import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:flutter/material.dart';
import 'package:plugin/generated/rid_api.dart' as rid;
import 'package:moon/providers/store_provider.dart';

class GraphSelection extends HookConsumerWidget {
  late final List<rid.GraphEntry> graphList;
  GraphSelection(this.graphList);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    // AsyncValue<rid.Store> nodeProvider = ref.watch(storeStreamProvider);
    final store = ref.watch(storeRepoProvider);

    final graphs = graphList.map((graph_id) {
      return GestureDetector(
        onTap: () {
          store.store.msgLoadGraph(graph_id.id);
        },
        child: Container(
          child: Text(graph_id.name),
          width: 100,
          height: 100,
          decoration: BoxDecoration(
            color: Color(0xFFEEEEEE),
          ),
        ),
      );
    }).toList();

    return Container(
      child: Center(
        child: Padding(
          padding: EdgeInsetsDirectional.fromSTEB(20, 20, 20, 20),
          child: GridView(
            padding: EdgeInsets.zero,
            gridDelegate: SliverGridDelegateWithFixedCrossAxisCount(
              crossAxisCount: 5,
              crossAxisSpacing: 10,
              mainAxisSpacing: 10,
              childAspectRatio: 1,
            ),
            scrollDirection: Axis.vertical,
            children: [
              AddNewGraph(),
              ...graphs,
            ],
          ),
        ),
      ),
    );
  }
}

class AddNewGraph extends HookConsumerWidget {
  AddNewGraph({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final store = ref.watch(storeRepoProvider);
    return GestureDetector(
      onTap: () {
        store.store.msgLoadGraph("new");
      },
      child: Container(
        child: Text("add graph"),
        width: 100,
        height: 100,
        decoration: BoxDecoration(
          color: Color(0xFFEEEEEE),
        ),
      ),
    );
  }
}
