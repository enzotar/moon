// import 'package:flutter/material.dart';
// import 'package:flutter_hooks/flutter_hooks.dart';

// class FocusNodeManager {
//   FocusNodeManager._privateConstructor();
//   static final FocusNodeManager instance =
//       FocusNodeManager._privateConstructor();

//   List<String> map = ['main'];

//   addNode(String id) {
//     if (map.contains(id)) return;
//     map.add(id);
//     print('adding node id: $id');
//     //_focusNode.add(AppNode(id, node));
//   }

//   String getNode(String id) {
//     //print('requesting node id: $id');
//     return map.firstWhere((element) => element == id);
//     //return _focusNode.lastWhere((element) => element.id == id);
//   }

//   // AppNode getMainNode() =>
//   //     _focusNode.lastWhere((element) => element.id == 'main');

//   FocusNode getMainNode() => map['main']!;

//   requestFocus(BuildContext context, FocusNode node) {
//     print(map);
//     try {
//       map.forEach((k, v) {
//         if (v == node)
//           print('requesting focus for id $k');
//         else
//           print('not requested for $k');
//       });
//       FocusScope.of(context).requestFocus(node);
//     } catch (e) {}
//   }

//   requestMainFN(BuildContext context) {
//     try {
//       FocusScope.of(context).requestFocus(map['main']!);
//     } catch (e) {}
//   }

//   removeFocus(BuildContext context) {
//     // FocusScope.of(context).requestFocus(map['main']!);

//     FocusScope.of(context).unfocus();
//     // FocusNodeManager.instance
//     //     .requestFocus(context, FocusNodeManager.instance.getMainNode().id);
//   }
// }
