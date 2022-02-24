import 'package:flutter/material.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:plugin/generated/rid_api.dart' as rid;
import 'package:moon/providers/store_provider.dart';

class CommandWidget extends HookConsumerWidget {
  CommandWidget({
    Key? key,
    required this.node,
    required this.selected,
    required this.inputs,
    required this.outputs,
    required this.label,
  }) : super(key: key);

  final MapEntry<String, rid.NodeView> node;
  final bool selected;
  final List<Widget> inputs;
  final List<Widget> outputs;
  final String label;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final provider = ref.watch(nodeProvider);
    final nodeFrom = provider.entries.firstWhere((element) {
      return element.key == this.node.key;
    });
    return Column(
      children: [
        Container(
          height: 30,
          child: Padding(
            padding: EdgeInsets.fromLTRB(0, 0, 0, 5),
            child: Text(
              label,
              style: TextStyle(
                  color: Colors.white,
                  fontSize: 22.00,
                  fontWeight: FontWeight.bold),
            ),
          ),
        ),
        Stack(clipBehavior: Clip.none, children: [
          Row(
            children: [
              Container(width: 15, color: Colors.transparent),
              Card(
                color: node.value.success == "success"
                    ? Color(Colors.green.value)
                    : (node.value.success == "fail"
                        ? Color(Colors.red.value)
                        : Color(0xFFEEEEEE)),
                margin: EdgeInsets.fromLTRB(0, 0, 0, 0),
                child: Container(
                  //adjustment for input size
                  width: node.value.width.toDouble() - 30,
                  height: node.value.height.toDouble() - 30,
                ),
              ),
              Container(width: 15, color: Colors.transparent),
            ],
          ),
          Positioned(
            left: 0,
            child: Column(
              mainAxisSize: MainAxisSize.max,
              children: inputs,
            ),
          ),
          Positioned(
            right: 0,
            child: Column(
              mainAxisSize: MainAxisSize.max,
              children: outputs,
            ),
          )
        ]),
      ],
    );
  }
}
