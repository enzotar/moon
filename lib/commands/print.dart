import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:plugin/generated/rid_api.dart' as rid;
import 'package:rheetah/providers/store_provider.dart';

class Print extends HookConsumerWidget {
  Print({Key? key, required this.treeNode})
      : this.input = "",
        super(key: key);

  final String input;
  final TreeNode treeNode;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    Future<void> _copyToClipboard(text) async {
      await Clipboard.setData(ClipboardData(text: text));
      ScaffoldMessenger.of(context).showSnackBar(SnackBar(
        content: Text('Copied to clipboard', textAlign: TextAlign.center),
      ));
      // Scaffold.of(context).showSnackBar(snackbar)
    }

    return Container(
      child: Center(
        child: treeNode.node.value.success == "success"
            ? ListTile(
                trailing: IconButton(
                    icon: Icon(Icons.copy),
                    onPressed: () {
                      _copyToClipboard(treeNode.node.value.printOutput);
                    }),
                title: SelectableText(
                  treeNode.node.value.printOutput,
                  textAlign: TextAlign.right,
                  style: TextStyle(fontSize: 18),
                ),
              )
            : treeNode.node.value.success == "fail"
                ? ListTile(
                    trailing: IconButton(
                        icon: Icon(Icons.copy),
                        onPressed: () {
                          _copyToClipboard(treeNode.node.value.printOutput);
                        }),
                    title: SelectableText(
                      treeNode.node.value.error,
                      textAlign: TextAlign.right,
                      style: TextStyle(fontSize: 18),
                    ),
                  )
                : null,
      ),
    );
  }
}


/*
Card(
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

 */