import 'package:flutter/material.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

import 'package:plugin/generated/rid_api.dart' as rid;
import 'package:moon/providers/store_provider.dart';
import 'package:tuple/tuple.dart';

HookConsumerWidget PortEntry(
  inputType,
  nodeEntry,
) {
  HookConsumerWidget? _widget;

  switch (inputType) {
    case "String":
      {
        _widget = TextEntry(inputType, nodeEntry);
      }
      break;
    case "Number":
    case "wait":
    case "Pubkey":
      {
        _widget = BasicPort(inputType, nodeEntry);
      }
      break;

    default:
      {
        _widget = BasicPort(inputType, nodeEntry);
      }
      print(inputType);
  }

  return _widget as HookConsumerWidget;
}

class TextEntry extends HookConsumerWidget {
  TextEntry(
    this.inputType,
    this.nodeEntry, {
    Key? key,
  }) : super(key: key);

  final Tuple2<String, rid.NodeView> nodeEntry;
  final String inputType;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    // final itemFocusNode = useFocusNode();
    // // listen to focus chances
    // useListenable(itemFocusNode);
    // final isFocused = itemFocusNode.hasFocus;

    final textEditingController = useTextEditingController();

    final textFieldFocusNode = useFocusNode();

    return Expanded(
      child: Padding(
          padding: const EdgeInsets.fromLTRB(5, 0, 0, 0),
          child: TextField(
            focusNode: textFieldFocusNode,
            // autofocus: true,
            decoration: InputDecoration(
              focusedBorder: UnderlineInputBorder(),
              enabledBorder: InputBorder.none,
              labelText: nodeEntry.item2.text,
            ),
            controller: textEditingController,
            style: TextStyle(),
            maxLines: 1,
          )),
    );
  }
}

/*
class TextEntry extends HookConsumerWidget {
  TextEntry(
    this.inputType,
    this.nodeEntry, {
    Key? key,
  }) : super(key: key);

  final Tuple2<String, rid.NodeView> nodeEntry;
  final String inputType;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final itemFocusNode = useFocusNode();
    // listen to focus chances
    useListenable(itemFocusNode);
    final isFocused = itemFocusNode.hasFocus;

    final textEditingController = useTextEditingController();

    final textFieldFocusNode = useFocusNode();

    return Expanded(
      child: Padding(
          padding: const EdgeInsets.fromLTRB(5, 0, 0, 0),
          child: Focus(
            focusNode: itemFocusNode,
            onFocusChange: (focused) {
              if (focused) {
                // textEditingController.text = todo.description;
              } else {
                // Commit changes only when the textfield is unfocused, for performance
                // ref
                //     .read(todoListProvider.notifier)
                //     .edit(id: todo.id, description: textEditingController.text);
              }
            },
            child: ListTile(
              onTap: () {
                itemFocusNode.requestFocus();
                textFieldFocusNode.requestFocus();
                print(itemFocusNode);
                print(textFieldFocusNode);
                print(FocusManager.instance.primaryFocus);
              },
              title: isFocused
                  ? TextField(
                      focusNode: textFieldFocusNode,
                      autofocus: true,
                      // decoration: InputDecoration(
                      //   focusedBorder: UnderlineInputBorder(),
                      //   enabledBorder: InputBorder.none,
                      //   labelText: nodeEntry.item2.text,
                      // ),
                      controller: textEditingController,
                      style: TextStyle(),
                      maxLines: 1,
                    )
                  : Text(nodeEntry.item2.text),
            ),
          )),
    );
  }
}
 */

class BasicPort extends HookConsumerWidget {
  BasicPort(
    this.inputType,
    this.nodeEntry, {
    Key? key,
  }) : super(key: key);

  final Tuple2<String, rid.NodeView> nodeEntry;
  final String inputType;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return Expanded(
      child: Padding(
        padding: const EdgeInsets.fromLTRB(5, 0, 0, 0),
        child: Text(
          "${nodeEntry.item2.text}", //${inputName}",
          style: TextStyle(),
          maxLines: 2,
          softWrap: true,
        ),
      ),
    );
  }
}
