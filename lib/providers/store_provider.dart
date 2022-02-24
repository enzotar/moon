import 'dart:collection';
import 'dart:async';

import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:plugin/generated/rid_api.dart';

abstract class BaseStoreRepo {}

final storeRepoProvider = Provider<StoreRepo>((ref) {
  return StoreRepo(ref.read);
});

class StoreRepo {
  StoreRepo(this._read) : this.store = Store.instance;

  final Reader _read;

  final Store store;
}

///
final canvasProvider = StateNotifierProvider<CanvasController, View>(
    (ref) => CanvasController(ref));

///
class CanvasController extends StateNotifier<View> {
  CanvasController(this._ref) : super(Store.instance.view) {
    _subscribe();
    // _refresh();
  }

  final Ref _ref;
  StreamSubscription<PostedConfirm>? _streams;

  _refresh() {
    // Store.instance.last_view_changes.changed_nodes_ids
    // Confirm.RefreshNode
    state = Store.instance.view;
  }

  void _subscribe() {
    final _streams = rid.replyChannel.stream;

    _streams.listen((ev) {
      switch (ev.type) {
        case Confirm.RefreshUI:
          {
            final changes = Store.instance.lastViewChanges;

            // FIXME: Workaround for changedNodesIds
            if (changes.changedNodesIds.isNotEmpty) {
              _ref.read(nodeProvider.notifier).refresh(changes.changedNodesIds);
            }
            if (changes.changedNodesIds.isNotEmpty &&
                changes.changedFlowEdgesIds.isNotEmpty) {
              // _ref.read(nodeProvider.notifier).refresh(changes.changedNodesIds); // twice
              _ref
                  .read(edgeProvider.notifier)
                  .refresh(changes.changedFlowEdgesIds); // check dummy
            }

            /*
            if (changes.changedNodesIds.isNotEmpty) {
              _ref.read(nodeProvider.notifier).refresh(changes.changedNodesIds);
            }
            if (changes.changedNodesIds.isNotEmpty &&
                changes.changedFlowEdgesIds.isNotEmpty) {
              // _ref.read(nodeProvider.notifier).refresh(changes.changedNodesIds); // twice
              _ref
                  .read(edgeProvider.notifier)
                  .refresh(changes.changedFlowEdgesIds); // check dummy
            }
            */

            // if (changes.changedFlowEdgesIds.isNotEmpty) {
            //   print("dummy_edge");

            //   _ref
            //       .read(edgeProvider.notifier)
            //       .refresh(changes.changedFlowEdgesIds);
            // }
          }
          break;
        // case Confirm.RefreshUI:
        // case Confirm.RefreshUI:

        case Confirm.ApplyCommand: //must recreate tree
        case Confirm.CreateNode: //
        case Confirm.RemoveNode: //
        case Confirm.LoadGraph:
        case Confirm.Initialized:
          {
            _refresh();
            // workaround, call the other providers?
            _ref.read(nodeProvider.notifier).reset();
            _ref.read(edgeProvider.notifier).reset();
          }
          break;
        default:
      }

      // _refresh();
    });
  }
}

///
final nodeProvider =
    StateNotifierProvider<NodeController, HashMap<String, NodeView>>(
        (ref) => NodeController(ref));

class NodeController extends StateNotifier<HashMap<String, NodeView>> {
  NodeController(this._ref) : super(Store.instance.view.nodes) {
    // _subscribe();
    // _refresh();
  }

  // _subscribe() {
  //   final view = Store.instance.view;
  //   _ref.listen(canvasProvider, (view, view) => refresh());
  // }

  final Ref _ref;
  StreamSubscription<PostedConfirm>? _streams;

  refresh(node_ids) {
    state = Store.instance.view.nodes;
  }

  reset() {
    state = Store.instance.view.nodes;
  }
}

///
final edgeProvider =
    StateNotifierProvider<EdgeController, HashMap<String, EdgeView>>(
        (ref) => EdgeController(ref));

class EdgeController extends StateNotifier<HashMap<String, EdgeView>> {
  EdgeController(this._ref) : super(Store.instance.view.flowEdges) {
    // _subscribe();
    // _refresh();
  }

  final Ref _ref;
  StreamSubscription<PostedConfirm>? _streams;

  refresh(edge_ids) {
    state = Store.instance.view.flowEdges;
  }

  reset() {
    state = Store.instance.view.flowEdges;
  }
}

// final storeStreamProvider = StreamProvider.autoDispose<Store>(
//   (ref) async* {
//     Store store = Store.instance;

//     final channel = rid.replyChannel.stream.where((event) {
//       // print(event);
//       return event.type == Confirm.RefreshUI ||
//           event.type == Confirm.Initialized ||
//           event.type == Confirm.LoadGraph ||
//           event.type == Confirm.ApplyCommand;
//     });

//     final stream = channel.listen((event) {
//       store = Store.instance;
//     });

//     ref.onDispose(() {
//       stream.cancel();
//     });

//     await for (final _ in channel) {
//       yield store;
//     }
//   },
// );

// final updateStoreStreamProvider = StreamProvider.autoDispose<Store>(
//   (ref) async* {
//     Store store = Store.instance;

//     final channel = rid.replyChannel.stream.where((event) {
//       // print(event);
//       return event.type == Confirm.UpdateStore;
//     });

//     final stream = channel.listen((event) {
//       store = Store.instance;
//     });

//     ref.onDispose(() {
//       stream.cancel();
//     });

//     await for (final _ in channel) {
//       yield store;
//     }
//   },
// );

final refreshStreamProvider = StreamProvider.autoDispose<Store>(
  (ref) async* {
    Store store = Store.instance;

    final channel = rid.replyChannel.stream.where((event) {
      // print(event);
      return event.type == Confirm.Refresh;
    });

    final stream = channel.listen((event) {
      store.msgRefresh("refresh");
      // store = Store.instance;
    });

    ref.onDispose(() {
      stream.cancel();
    });

    // await for (final _ in channel) {
    //   yield store;
    // }
  },
);

// final nodeStreamProvider =
//     StreamProvider<HashMap<String, NodeView>>((ref) async* {
//   Store store = Store.instance;

//   final channel = rid.replyChannel.stream.where((event) => event.type = Confirm.RefreshNodes)
// });

// sttate.selection

// final nodeProvider = FutureProvider<Store>((ref) async {
//   Store store = Store.instance;

//   store = await ref.watch(nodeStreamProvider.future);

//   return store;
// });
