// import 'dart:async';

// import 'package:bloc/bloc.dart';
// import 'package:meta/meta.dart';
// import 'package:plugin/generated/rid_api.dart';

// part 'posts_state.dart';

// class PostsCubit extends Cubit<NodeState> {
//   final _store = Store.instance;
//   StreamSubscription<PostedConfirm>? _nodeAddedOrRemovedSub;

//   PostsCubit() : super(NodeState([])) {
//     _subscribe();
//     _refresh();
//   }

//   void _subscribe() {
//     _nodeAddedOrRemovedSub = rid.replyChannel.stream
//         .where((event) => event.type == Confirm.RefreshUI)
//         .listen((_) => _refresh());
//   }

//   Future<void> _unsubscribe() async {
//     await _nodeAddedOrRemovedSub?.cancel();
//     _nodeAddedOrRemovedSub = null;
//   }

//   void _refresh() {
//     final nodes = _store.uiStore.view.nodes.values.toList();
//     // Show most recently added post first
//     // posts.sort((a, b) => a.scores.length.compareTo(b.scores.length));
//     emit(NodeState(nodes));
//   }

//   @override
//   Future<void> close() async {
//     await _unsubscribe();
//     return super.close();
//   }
// }
