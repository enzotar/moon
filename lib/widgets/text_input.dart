import 'package:dart_json_mapper/dart_json_mapper.dart';
import 'package:flutter/scheduler.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:plugin/generated/rid_api.dart' as rid;
import 'package:flutter/material.dart';
import 'package:moon/providers/store_provider.dart';
import 'package:moon/serialization/input_mapping.dart';

/// Must call ApplyCommand in two places
///
/// 1. onFieldSubmitted; where user types
/// 2. InkWell, onTap; where user clicks on autocomplete selection
///
///
class TextInput extends HookConsumerWidget {
  TextInput({
    Key? key,
    BuildContext? context,
    required this.node,
    required this.children,
    // required this.selectedNode,
    required this.selected,
  }) : super(key: key);

  final MapEntry<String, rid.NodeView> node;
  final List<Widget> children;
  final bool selected;
  // final FocusNode selectedNode;

  // final FocusNode focusNode;
  // final TextEditingController _controller;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final store = ref.watch(storeRepoProvider);
    final view = ref.watch(nodeProvider);
    final FocusNode focusNode = useFocusNode();

    List<rid.WidgetTextCommand> _userOptions = store.store.view.textCommands;

    _userOptions.sort((a, b) {
      return a.commandName.toLowerCase().compareTo(b.commandName.toLowerCase());
    });

    String _displayStringForOption(rid.WidgetTextCommand option) =>
        option.commandName;

    // final TextEditingController _controller =
    //     useTextEditingController(text: node.value.text);

    final double optionsMaxHeight = 200;
    final double optionsMaxWidth = node.value.width.toDouble();

    return Positioned(
      height: node.value.height.toDouble(),
      width: node.value.width.toDouble(),
      left: node.value.x.toDouble(),
      top: node.value.y.toDouble(),
      child: Card(
        child: Stack(
          children: [
            Text(node.key),
            Padding(
              padding: const EdgeInsets.all(8.0),
              child: Autocomplete(
                displayStringForOption: _displayStringForOption,
                fieldViewBuilder: (BuildContext context,
                    TextEditingController textEditingController,
                    FocusNode focusNode,
                    VoidCallback onFieldSubmitted) {
                  textEditingController.value =
                      TextEditingValue(text: node.value.text);

                  return TextFormField(
                    decoration: const InputDecoration(
                      hintText: 'press / for commands',
                    ),
                    controller:
                        textEditingController, // check controller and focus
                    focusNode: focusNode,
                    autofocus: true,
                    onTap: () {
                      textEditingController.selection =
                          TextSelection.fromPosition(
                              TextPosition(offset: node.value.text.length));
                    },
                    onChanged: (text) {
                      // textEditingController.value = TextEditingValue(text: text);
                      // print(text);
                    },
                    onEditingComplete: () {},
                    onFieldSubmitted: (String value) {
                      onFieldSubmitted();
                      final commandName = textEditingController.text;

                      // prevent non-existent command from being called
                      final match = _userOptions.where(((textCommand) {
                        return textCommand.commandName == commandName;
                      }));
                      if (match.isNotEmpty &&
                          commandName == match.first.commandName) {
                        focusNode.unfocus();
                        store.store
                            .msgApplyCommand(commandName); // call ApplyCommand
                      }

                      if (match.isEmpty) {
                        final text = textEditingController.value.text;
                        print(text);
                        final inputProperties = {
                          "nodeId": node.key,
                          "text": text
                        };
                        String inputEvent = JsonMapper.serialize(
                            InputProperties(inputProperties));
                        store.store.msgSetText(inputEvent);
                      }
                    },
                  );
                },
                optionsBuilder: ((textEditingValue) {
                  if (textEditingValue.text == '') {
                    return const Iterable<rid.WidgetTextCommand>.empty();
                  }
                  if (textEditingValue.text.startsWith('/')) {
                    // remove slash and pass to options
                    final newTextEditingValue = textEditingValue.replaced(
                        TextRange(start: 0, end: 1), "");
                    return _userOptions.where((rid.WidgetTextCommand option) {
                      return option
                          .toString()
                          .contains(newTextEditingValue.text.toLowerCase());
                    });
                  } else {
                    return const Iterable<rid.WidgetTextCommand>.empty();
                  }
                }),
                optionsViewBuilder: (BuildContext context,
                    AutocompleteOnSelected<rid.WidgetTextCommand> onSelected,
                    Iterable<rid.WidgetTextCommand> options) {
                  return Align(
                    alignment: Alignment.topLeft,
                    child: Material(
                      elevation: 4.0,
                      child: ConstrainedBox(
                        constraints: BoxConstraints(
                            maxHeight: optionsMaxHeight,
                            maxWidth: optionsMaxWidth),
                        child: ListView.builder(
                          padding: EdgeInsets.zero,
                          shrinkWrap: true,
                          itemCount: options.length,
                          itemBuilder: (BuildContext context, int index) {
                            final rid.WidgetTextCommand option =
                                options.elementAt(index);
                            return InkWell(
                              onTap: () {
                                onSelected(option);
                                store.store.msgApplyCommand(
                                    option.commandName); // call ApplyCommand
                              },
                              child: Builder(
                                builder: (BuildContext context) {
                                  final bool highlight =
                                      AutocompleteHighlightedOption.of(
                                              context) ==
                                          index;
                                  if (highlight) {
                                    SchedulerBinding.instance!
                                        .addPostFrameCallback(
                                      (Duration timeStamp) {
                                        Scrollable.ensureVisible(context,
                                            alignment: 0.5);
                                      },
                                    );
                                  }
                                  return Container(
                                    color: highlight
                                        ? Theme.of(context).focusColor
                                        : null,
                                    padding: const EdgeInsets.all(16.0),
                                    child:
                                        Text(_displayStringForOption(option)),
                                  );
                                },
                              ),
                            );
                          },
                        ),
                      ),
                    ),
                  );
                },
              ),
            )
          ],
        ),
      ),
    );
  }
}

/*
TextField(
            autofocus: false,
            focusNode: focusNode,
            controller: _controller,
            onChanged: (value) {
              //disposing focusnode while being used
            },
            onSubmitted: (value) {
              final inputProperties = {
                "nodeId": node.key,
                "text": value,
              };
              String inputEvent =
                  JsonMapper.serialize(InputProperties(inputProperties));
              store.store.msgSetText(inputEvent);
            },
            onTap: () {
              focusNode.requestFocus();
              // FocusNodeManager.instance.removeFocus(context);
              // FocusNodeManager.instance.requestFocus(context, selectedNode);
            },

            // onEditingComplete: () =>
            //     Focus.of(context).ancestors.first.requestFocus(),
          ),
 */
