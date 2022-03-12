import 'package:flutter/material.dart';

import 'package:hooks_riverpod/hooks_riverpod.dart';

import 'package:moon/providers/bookmark.dart';
import 'package:moon/providers/store_provider.dart';
import 'package:plugin/generated/rid_api.dart' as rid;

class BookmarkManager extends HookConsumerWidget {
  const BookmarkManager({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final bookmarks = ref.watch(bookmarkController);
    final store = ref.read(storeRepoProvider).store;

    return Container(
      width: 300,
      decoration: BoxDecoration(color: Colors.blueGrey[800]),
      child: ListView.separated(
        separatorBuilder: ((context, index) => const Divider()),
        itemBuilder: ((context, index) {
          final bookmark = bookmarks.entries.elementAt(index);

          return GestureDetector(
              onTap: () {
                store.msgGotoBookmark(bookmark.key);
              },
              child: Container(
                decoration: BoxDecoration(color: Colors.black12),
                child: Dismissible(
                  background: Container(
                    alignment: Alignment.centerRight,
                    color: Colors.red,
                    child: Padding(
                      padding: const EdgeInsets.fromLTRB(0, 0, 10, 0),
                      child: Icon(Icons.cancel),
                    ),
                  ),
                  direction: DismissDirection.endToStart,
                  onDismissed: (direction) {
                    store.msgDeleteBookmark(bookmark.key);
                    ScaffoldMessenger.of(context).showSnackBar(SnackBar(
                      content:
                          Text('Deleted bookmark', textAlign: TextAlign.center),
                    ));
                  },
                  key: UniqueKey(),
                  child: Padding(
                    padding: const EdgeInsets.all(8.0),
                    child: Text(bookmark.value.nodes.toString()),
                  ),
                ),
              ));
        }),
        itemCount: bookmarks.length,
        // children: commandList,
      ),
    );
  }
}
