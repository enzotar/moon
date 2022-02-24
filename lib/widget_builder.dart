import 'dart:collection';

import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:plugin/generated/rid_api.dart' as rid;
import 'package:moon/nodes/add_port.dart';
import 'package:moon/widget_chooser.dart';
import 'logger.dart';
import 'package:flutter/material.dart';
import 'package:tuple/tuple.dart';

/// Serve Data Function
/// Given a list of head nodes and filters
/// returns a widget tree with all children
///
List<dynamic> returnWidgetTreeFunction(
  HashMap<String, rid.NodeView> nodes,
  HashMap<String, rid.NodeView> vertexNodes,
  rid.View store,
) {
  /// Visited Elements
  HashMap<String, rid.EdgeView> visitedEdgeElements = HashMap();
  HashMap<String, rid.NodeView> visitedNodeViews = HashMap();
  List<HookConsumerWidget> nodeWidgets = [];

  /// Function to build widgets recursively if they have children
  ///
  List<HookConsumerWidget> buildWidgetTree(
      HashMap<String, rid.NodeView> inputNodes) {
    List<HookConsumerWidget> widgetList = [];

    final List<String> nodeIds = inputNodes.keys.toList();
    nodeIds.sort();

    nodeIds.forEach(
      (nodeId) {
        final nodeView = inputNodes[nodeId]!;
        final nodeEntry = MapEntry<String, rid.NodeView>(nodeId, nodeView);

        visitedNodeViews.putIfAbsent(nodeId, () => nodeView);

        HashMap<String, rid.EdgeView> edgeWithChild = HashMap.fromEntries(
          nodeView.outboundEdges.entries
              // // .where((element) => element.edgeType != rid.ViewEdgeType.Flow)
              // // .where((element) => element.facets["expired"] != true)
              .where(
            (edgeEntry) => !visitedEdgeElements.containsKey(edgeEntry.key),
          ),
        );
        //     .where(
        //   (e) {
        //     return _filterList.every(
        //       (element) {
        //         return element(e);
        //       },
        //     );
        //   },
        // )
        if (edgeWithChild.isNotEmpty) {
          visitedEdgeElements.addAll(edgeWithChild);

          edgeWithChild.removeWhere((edgeKey, edgeView) =>
              edgeView.edgeType == rid.ViewEdgeType.Flow);

          HashMap<String, rid.NodeView> toVisitNodes = HashMap();

          edgeWithChild.forEach(
            (edgeKey, edgeView) {
              final node_to = nodes.remove(edgeView.to)!;
              toVisitNodes.putIfAbsent(edgeView.to, () => node_to);
            },
          );
          // Block
          final HookConsumerWidget widget = WidgetChooser(
              nodeView.widgetType.name,
              nodeEntry,
              buildWidgetTree(toVisitNodes),
              null);

          widgetList.add(widget);
          nodeWidgets.add(widget);
        } else {
          // FocusNodeManager.instance.addNode(nodeId);
          // final selectedNode = FocusNodeManager.instance.getNode(nodeId);

          // TODO separate for TextInput and Commands

          // find inputs and outputs
          // search through all nodes for the input/output widgets

          final HashMap<String, rid.NodeView> portNodes =
              HashMap.fromEntries(nodes.entries.where(((element) {
            return element.value.parentId == nodeId;
          })));

          // print(nodeId);
          // print(portNodes);

          var inputMap = SplayTreeMap<int, Tuple2<String, rid.NodeView>>();

          portNodes.entries.where(((element) {
            return element.value.widgetType.name == "WidgetInput";
          })).forEach((element) => inputMap[element.value.index] =
              Tuple2(element.key, element.value));

          // print(inputMap);

          var outputMap = SplayTreeMap<int, Tuple2<String, rid.NodeView>>();

          portNodes.entries.where(((element) {
            return element.value.widgetType.name == "WidgetOutput";
          })).forEach((element) => outputMap[element.value.index] =
              Tuple2(element.key, element.value));

          // print(outputMap);

          // find highlighted ports
          List<String> highlightedPort = store.highlighted;

          //pass command name to inputs
          final String command_name = nodeEntry.value.widgetType.name;

          // build the widget
          final inputs =
              addPort(inputMap, highlightedPort, store, command_name);
          final outputs =
              addPort(outputMap, highlightedPort, store, command_name);
          // pass it to the command

          /// Text Input or Command
          final HookConsumerWidget widget = WidgetChooser(
            nodeView.widgetType.name,
            nodeEntry,
            null,
            inputs,
            outputs,
          );

          widgetList.add(widget);
          nodeWidgets.add(widget);
        }
      },
    );

    ///
    return widgetList;
  }

  List<HookConsumerWidget> result = buildWidgetTree(vertexNodes);

  /// reset visited edges
  final isVisitedEdgeElements = visitedEdgeElements;
  final isVisitedNodeElements = visitedNodeViews;
  final isWidgets = nodeWidgets;

  visitedEdgeElements = HashMap();
  visitedNodeViews = HashMap();
  nodeWidgets = [];

  return [
    result,
    isVisitedEdgeElements,
    isVisitedNodeElements,
    isWidgets,
  ];
}

// rebuildAndUpdateVolume() {
//   tools.log.i("/// START Build and Update Volume");

//   /// get volume from memory
//   final NodeElement _currentVolume = sl<MemoryDatabase>().currentVolume;

//   /// find all top blocks
//   final List<NodeElement> headNodes = _currentVolume.edges
//       .where((element) =>
//           element.facets['relationship'] == "child" &&
//           element.facets['expired'] != true)
//       .map((element) => element.nodeList[1])
//       .where((element) => element.properties['type'] == "block")
//       .toList();

//   /// build widgets tree from the top nodes
//   final result = returnWidgetTreeFunction(headNodes, filterListForVolume);
//   final List<NodeElement> resultNodeElements = result[2];

//   /// find sibling edges
//   List<EdgeElement> siblingEdges = sl<MemoryDatabase>()
//       .getEdgeElements
//       .where((edgeElement) =>
//           resultNodeElements.contains(edgeElement.nodeList[0]) ||
//           resultNodeElements.contains(edgeElement.nodeList[1]))
//       .where((edgeElement) => edgeElement.facets["relationship"] == "link")
//       .where((edgeElement) =>
//           edgeElement.nodeList[0].properties["type"] == "block")
//       .toList();

//   /// build sibling widgets
//   List<SuperStatefulWidget> siblingWidgets =
//       siblingEdges.map((e) => addEdgeWidget(e)).toList();

//   /// save snapshot to memory
//   final VolumeMemory volumeMemory = VolumeMemory(
//     volumeName: _currentVolume.properties["name"],
//     nodeElements: result[2],
//     edgeElements: [...result[1], ...siblingEdges],
//     nodeWidgets: result[0],
//     edgeWidgets: siblingWidgets,
//     widgetList: result[3],
//     inputNodes: headNodes,
//   );

//   sl<MemoryDatabase>().updateCurrentVolume(volumeMemory);

//   tools.log.i("/// END Build and Update Volume");
// }

// ///
// ///
// ///
// ///
// rebuildAndUpdateDataForVolume() {
//   tools.log.v("/// START rebuild and update data for volume");

//   final NodeElement _currentVolume = sl<MemoryDatabase>().currentVolume;
//   tools.log.v("/// ${_currentVolume.properties["name"]}");

//   ///
//   NodeElement headNode = getNodesOfType("data")
//       .where((element) =>
//           element.properties["name"] == _currentVolume.properties["name"])
//       .first;

//   final result2 = traverseBreadth(headNode, includeSelf: true);
//   final List<NodeElement> resultNodes = result2[0];
//   final List<EdgeElement> resultEdges = result2[1];
//   final resultNodeWidget = resultNodes.map((e) => addNodeWidget(e)).toList();
//   final resultEdgeWidget = resultEdges.map((e) => addEdgeWidget(e)).toList();

//   /// build sibling edges from data edges
//   ///
//   List<EdgeElement> siblingEdges = sl<MemoryDatabase>()
//       .getEdgeElements
//       .where((element) => element.facets["relationship"] == "link")
//       .where((element) => element.nodeList[0].properties["type"] != "block")
//       .toList();

//   List<SuperStatefulWidget> siblingWidgets =
//       siblingEdges.map((e) => addEdgeWidget(e)).toList();

//   final VolumeMemory volumeMemory = VolumeMemory(
//     volumeName: _currentVolume.properties["name"],
//     dataNodeElements: resultNodes,
//     dataEdgeElements: [...resultEdges, ...siblingEdges],
//     dataNodeWidgets: resultNodeWidget,
//     dataEdgeWidgets: [...resultEdgeWidget, ...siblingWidgets],
//   );

//   // volumeMemory.dataEdgeWidgets = volumeMemory.dataEdgeElements.map(
//   //   (edgeElement) {
//   //     return addEdgeWidget(edgeElement);
//   //   },
//   // ).fold(
//   //   [],
//   //   (List<SuperStatefulWidget> previousValue, element) {
//   //     previousValue.add(element);
//   //     return previousValue;
//   //   },
//   // );

//   sl<MemoryDatabase>().updateDataForCurrentVolume(volumeMemory);

//   tools.log.v("/// END rebuild and update data for volume");
// }

// abstract class NodeEntryDisplay {
//   static Function displayFromString(String? source) {
//     if (source == null) {
//       return (List<dynamic> inputs) => const NoSource();
//     } else {
//       final widgetStore = <String, Function>{
//         "listViewer": (List<dynamic> inputs) => ListViewer(inputs),
//         // "queryListViewer": (List<dynamic> inputs) => QueryListViewer(inputs),
//         "noSource": (List<dynamic> inputs) => const NoSource(),
//       };

//       return widgetStore.entries
//           .firstWhere((element) => element.key == source)
//           .value;
//     }
//   }

//   static Function inlineDisplayFromString(String? source) {
//     if (source == null) {
//       return (List<dynamic> inputs) => const NoSource();
//     } else {
//       final widgetStore = <String, Function>{
//         "noSource": (List<dynamic> inputs) => const NoSource(),
//         "textInput": (List<dynamic> inputs) => TextInputWidget(inputs),
//         "textButton": (List<dynamic> inputs) => TextButtonWidget(inputs),
//         "mapViewer": (List<dynamic> inputs) => MapViewer(inputs),
//       };

//       return widgetStore.entries
//           .firstWhere((element) => element.key == source)
//           .value;
//     }
//   }
// }

// class NodeToWidgetDisplayReader extends HookConsumerWidget {
//   @override
//   Widget build(BuildContext context, WidgetRef ref) {
//     final _repo = ref.watch(graphRepoControllerProvider);

//     return _repo.when(
//       data: (nodes) {
//         if (nodes.isEmpty) return const NoSource();

//         // passing _currentRootNode to satisfy arguments only
//         return nodes.first.display.call([nodes]) as Widget;
//       },
//       loading: () => const Center(
//         child: CircularProgressIndicator(),
//       ),
//       error: (e, st) => const Center(
//         child: Text('Something went wrong'),
//       ),
//     );
//   }
// }

// SuperStatefulWidget addNodeWidget(NodeElement node) {
//   final SuperStatefulWidget widget = WidgetChooser(
//     node.properties["type"],
//     node,
//   );
//   return widget;
// }

// ///
// ///
// /// Traverse graph and build it
// ///
// List<dynamic> traverseBreadth(vertexNode,
//     {bool includeSelf = false, bool expired = false, int maxLevel: 10000}) {
//   List<NodeElement> visitedNodes = [];

//   ///
//   List<NodeElement> nodeStack = [];
//   List<EdgeElement> visitedEdges = [];
//   int counter = 0;

//   nodeStack.add(vertexNode);

//   while (nodeStack.isNotEmpty) {
//     NodeElement node = nodeStack.first;
//     nodeStack.removeAt(0);
//     if (!visitedNodes.contains(node)) {
//       visitedNodes.add(node);

//       /// find edges and children
//       List<EdgeElement> edgeWithChild = node.edges
//           .where((element) => element.facets["relationship"] == "child")
//           .where((element) => element.facets["expired"] != expired)
//           .toList();

//       edgeWithChild.forEach(
//         (element) {
//           if (!visitedEdges.contains(element)) visitedEdges.add(element);
//           if (counter <= maxLevel) nodeStack.add(element.nodeList[1]);
//         },
//       );
//     }
//     counter += 1;
//   }
//   if (!includeSelf) visitedNodes.remove(vertexNode);
//   tools.log.v("visited $visitedNodes $visitedEdges ");

//   return [visitedNodes, visitedEdges];
// }


// // ///
// // ///
// // /// Filters
// // ///
// // bool filterNodeOutByName({
// //   required EdgeElement element,
// //   required String name,
// // }) {
// //   final filter = element.nodeList[0].properties["name"] != name;
// //   return filter;
// // }

// // bool filterEdgeWhereParentNodeIsNotOfType({
// //   required EdgeElement element,
// //   required String parentType,
// // }) {
// //   final filter = element.nodeList[0].properties["type"] != parentType;
// //   return filter;
// // }

// // bool filterEdgeByFacetRelationship({
// //   required EdgeElement element,
// //   required String facetRelationship,
// // }) {
// //   final filter = element.facets["relationship"] == facetRelationship;
// //   return filter;
// // }

// bool hasNotBeenVisited(
//   String edgeKey,
//   HashMap<String, rid.EdgeView> visitedEdgeElements,
// ) {
//   final filter = !visitedEdgeElements.containsKey(edgeKey);
//   return filter;
// }

// final filterList = [
//   (e) => filterEdgeByFacetRelationship(element: e, facetRelationship: "child"),
//   (e) => filterEdgeWhereParentNodeIsNotOfType(element: e, parentType: "volume"),
//   (e) => filterEdgeWhereParentNodeIsNotOfType(element: e, parentType: "user"),
// ];

// // final filterListForVolume = [
// //   // (e) => filterNodeOutByName(element: e, name: "home"),
// //   (e) => filterEdgeWhereParentNodeIsNotOfType(element: e, parentType: "volume"),
// //   (e) => filterEdgeByFacetRelationship(element: e, facetRelationship: "child"),
// //   (e) => filterEdgeWhereParentNodeIsNotOfType(element: e, parentType: "user"),
// // ];

// // final filterListForDataStructure = [
// //   (e) => filterNodeOutByName(element: e, name: "home"),
// //   (e) => filterEdgeByFacetRelationship(element: e, facetRelationship: "child"),
// //   (e) => filterEdgeWhereParentNodeIsNotOfType(element: e, parentType: "user"),
// // ];
