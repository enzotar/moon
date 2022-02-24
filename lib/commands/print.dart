import 'package:flutter/material.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:plugin/generated/rid_api.dart' as rid;

class Print extends HookConsumerWidget {
  Print({
    Key? key,
    required this.node,
    required this.selected,
    required this.inputs,
    required this.outputs,
  })  : this.input = "",
        super(key: key);

  final String input;
  final MapEntry<String, rid.NodeView> node;
  final bool selected;
  final List<Widget> inputs;
  final List<Widget> outputs;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return Positioned(
      // height: node.value.height.toDouble(),
      // width: node.value.width.toDouble(),
      // left: node.value.x.toDouble(),
      // top: node.value.y.toDouble(),
      child: Card(
        color: node.value.success == "success"
            ? Color(Colors.green.value)
            : Color(0xFFF5F5F5),
        child: Container(
          width: node.value.width.toDouble(),
          height: node.value.height.toDouble(),
          decoration: BoxDecoration(
            color: node.value.success == "success"
                ? Color(Colors.green.value)
                : Color(0xFFEEEEEE),
          ),
          child: Column(
            mainAxisSize: MainAxisSize.max,
            mainAxisAlignment: MainAxisAlignment.start,
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [
              Expanded(
                child: Row(
                  mainAxisSize: MainAxisSize.max,
                  mainAxisAlignment: MainAxisAlignment.spaceBetween,
                  crossAxisAlignment: CrossAxisAlignment.center,
                  children: [
                    Column(
                      mainAxisSize: MainAxisSize.max,
                      children: inputs,
                    ),
                    Expanded(
                      child: Padding(
                        padding: EdgeInsetsDirectional.fromSTEB(10, 5, 5, 10),
                        child: Column(
                          mainAxisSize: MainAxisSize.max,
                          mainAxisAlignment: MainAxisAlignment.start,
                          crossAxisAlignment: CrossAxisAlignment.stretch,
                          children: [
                            Container(
                              width: 400,
                              height: 75,
                              decoration: BoxDecoration(
                                color: Color(Colors.white.value),
                                border: Border.all(
                                  color: Color(0xFF258ED5),
                                ),
                              ),
                              child: Padding(
                                padding: EdgeInsetsDirectional.fromSTEB(
                                    10, 5, 5, 10),
                                child: Column(
                                  mainAxisSize: MainAxisSize.max,
                                  mainAxisAlignment: MainAxisAlignment.start,
                                  crossAxisAlignment: CrossAxisAlignment.start,
                                  children: [
                                    Padding(
                                      padding: EdgeInsetsDirectional.fromSTEB(
                                          5, 0, 5, 0),
                                      child: Text(
                                        '$input',
                                        // style: FlutterFlowTheme
                                        //     .bodyText1
                                        //     .override(
                                        //   fontFamily: 'Poppins',
                                        //   fontWeight: FontWeight.w600,
                                        // ),
                                      ),
                                    ),
                                  ],
                                ),
                              ),
                            ),
                          ],
                        ),
                      ),
                    ),
                  ],
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
