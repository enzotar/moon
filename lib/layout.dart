import 'dart:typed_data';

import 'package:dart_json_mapper/dart_json_mapper.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:flutter/rendering.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:recase/recase.dart';
import 'package:moon/bookmark_manager.dart';
import 'package:moon/commands/const_subblocks/file_picker.dart';
import 'package:moon/logger.dart';
import 'package:moon/providers/store_provider.dart';
import 'package:plugin/generated/rid_api.dart' as rid;
import 'package:moon/serialization/input_mapping.dart';
import 'package:flutter/services.dart';
import 'package:intl/intl.dart';

import 'event_listener.dart';
import 'package:file_picker/file_picker.dart';

class LayoutScreen extends HookConsumerWidget {
  static const routeName = "/Create";
  // LayoutScreen({Key? key}) : super(key: key);

  final TransformationController _transformationController =
      TransformationController();

  // double height = 3000;
  // double width = 3000;

  // https://api.flutter.dev/flutter/widgets/InteractiveViewer/transformationController.html
  // Animation<Matrix4>? _animationReset;
  // late final AnimationController _controllerReset;

  // void _onAnimateReset() {
  //   _transformationController.value = _animationReset!.value;
  //   if (!_controllerReset.isAnimating) {
  //     _animationReset!.removeListener(_onAnimateReset);
  //     _animationReset = null;
  //     _controllerReset.reset();
  //   }
  // }

  // void _onInteractionStart(ScaleStartDetails details) {
  //   // If the user tries to cause a transformation while the reset animation is
  //   // running, cancel the reset animation.
  //   if (_controllerReset.status == AnimationStatus.forward) {
  //     _animateResetStop();
  //   }
  // }

  // void _animateResetInitialize() {
  //   _controllerReset.reset();
  //   _animationReset = Matrix4Tween(
  //     begin: _transformationController.value,
  //     end: Matrix4.identity(),
  //   ).animate(_controllerReset);
  //   _animationReset!.addListener(_onAnimateReset);
  //   _controllerReset.forward();
  // }

  // // Stop a running reset to home transform animation.
  // void _animateResetStop() {
  //   _controllerReset.stop();
  //   _animationReset?.removeListener(_onAnimateReset);
  //   _animationReset = null;
  //   _controllerReset.reset();
  // }

  @override
  Widget build(BuildContext buildContext, WidgetRef ref) {
    // print("rebuilding layout");
    var _last = useState("");
    final provider = ref.watch(changesController);
    // final viewport = ref.watch(viewportController);
    final graphProvider = ref.watch(graphController);
    // print(provider);
    // final provider2 =
    //     ref.watch(changesController.select((value) => value.is_graph_changed));
    // print(provider2);
    final store = ref.read(storeRepoProvider).store;
    final graph_entry = ref.read(storeRepoProvider).graph_entry;

    // create dropdown options
    final List<rid.GraphEntry> graphList =
        ref.read(storeRepoProvider).graph_list;

    // remove current graph to re-add it at the top
    graphList.removeWhere((entry) => entry.id == graph_entry.id);

    List<DropdownMenuItem<String>> dropDownList = graphList.map(
      (e) {
        return DropdownMenuItem(child: Text(e.name), value: e.id);
      },
    ).toList();

    dropDownList.insert(
        0, DropdownMenuItem(child: Text("+ New Flow"), value: "new"));
    dropDownList.insert(1,
        DropdownMenuItem(child: Text(graph_entry.name), value: graph_entry.id));

    // create dropdown options
    final List<String> mainnet = ["Testnet", "Devnet", "Mainnet"];

    List<DropdownMenuItem<String>> mainnetList = mainnet.map(
      (e) {
        return DropdownMenuItem(child: Text(e), value: e);
      },
    ).toList();

    final mainnetSelection = useState(store.view.solanaNet.name.toString());

    ///
    final hideDrawer = useState(true);

    /// get list of commands
    ///

    final commands = ref.read(storeRepoProvider).text_commands;

    final commandList = commands.map(
      (e) {
        ReCase rc = ReCase(e.commandName);

        return Container(
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              ListTile(
                title: Text(rc.titleCase),
                textColor: Colors.white,
              ),
              Padding(
                padding: EdgeInsets.fromLTRB(10, 0, 0, 0),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text("command description......",
                        style: TextStyle(color: Colors.white)),
                    Draggable(
                      onDragEnd: (details) {
                        // print(details.offset);
                      },
                      child: Image.asset(
                        "assets/const.png",
                        // height: 100,
                      ),
                      feedback: Image.asset(
                        "assets/const.png",
                        // height: 100,
                      ),
                    )
                  ],
                ),
              )
            ],
          ),
        );
      },
    ).toList();

    commands.sort(((a, b) {
      return a.commandName.compareTo(b.commandName);
    }));

    // final transform = ref.read(storeRepoProvider).store.view.transform;

    // final transformController = useTransformationController();

    // transformController.value.
    final GlobalKey<ScaffoldState> _scaffoldKey = GlobalKey();

    /// Text renaming
    final renameTextEditingController =
        useTextEditingController(text: graph_entry.name);

    final graph_name = ValueNotifier(graph_entry.name);

    final update = useValueListenable(graph_name);

    useEffect(() {
      renameTextEditingController.text = update;
    }, [update]);

    final debug = useState("");

    Future<void> _copyToClipboard(text) async {
      await Clipboard.setData(ClipboardData(text: text));
      ScaffoldMessenger.of(buildContext).showSnackBar(SnackBar(
        content: Text('Copied to clipboard', textAlign: TextAlign.center),
      ));
      // Scaffold.of(context).showSnackBar(snackbar)
    }

// bookmark
    final selected_nodes_ids = ref.read(storeRepoProvider).selected_node_ids;

    final bookmarkTextController = useTextEditingController(text: "");

    return Scaffold(
      key: _scaffoldKey,
      drawer: Drawer(
        backgroundColor: Color.fromARGB(255, 23, 30, 34),
        child: ListView(
          children: [
            Image.asset("assets/logo-full-small.png"),
            Padding(
              padding: const EdgeInsets.all(8.0),
              child: Divider(color: Colors.white),
            ),
            TextButton.icon(
              style: TextButton.styleFrom(
                primary: Colors.blueGrey.shade400,
              ),
              icon: Icon(Icons.mouse),
              label: Text("Set Input For Mouse"),
              onPressed: () {
                ref.read(storeRepoProvider).store.msgSetMappingKind("mouse");
              },
            ),
            Padding(
              padding: const EdgeInsets.all(8.0),
              child: Divider(color: Colors.white),
            ),
            TextButton.icon(
              style: TextButton.styleFrom(
                primary: Colors.blueGrey.shade400,
              ),
              icon: Icon(Icons.laptop),
              label: Text("Set Input For Trackpad"),
              onPressed: () {
                ref.read(storeRepoProvider).store.msgSetMappingKind("touch");
              },
            ),
            Padding(
              padding: const EdgeInsets.all(8.0),
              child: Divider(color: Colors.white),
            ),
            TextButton.icon(
              style: TextButton.styleFrom(
                primary: Colors.blueGrey.shade400,
              ),
              icon: Icon(Icons.save),
              label: Text("export"),
              onPressed: () async {
                DateTime now = DateTime.now();
                String formattedDate =
                    DateFormat("yyyy-MM-dd--hhmmaa").format(now).toLowerCase();

                final filename = ref.read(storeRepoProvider).graph_entry.name +
                    " - " +
                    formattedDate;
                log.v(filename);
                String? path = await FilePicker.platform.saveFile(
                    fileName: filename,
                    type: FileType.custom,
                    allowedExtensions: ["json"]);

                if (path != null) {
                  log.v(path);
                  store.msgExport(path, filename);
                }
              },
            ),
            Padding(
              padding: const EdgeInsets.all(8.0),
              child: Divider(color: Colors.white),
            ),
            TextButton.icon(
              style: TextButton.styleFrom(
                primary: Colors.blueGrey.shade400,
              ),
              icon: Icon(Icons.folder_open_rounded),
              label: Text("import"),
              onPressed: () {
                filePicker(FileType.custom, jsonOnlyExtension,
                    PickerFollowAction.Import, ref);
              },
            ),
            Padding(
              padding: const EdgeInsets.all(8.0),
              child: Divider(color: Colors.white),
            ),
            TextButton.icon(
              style: TextButton.styleFrom(
                primary: Colors.blueGrey.shade400,
              ),
              icon: Icon(Icons.bug_report_outlined),
              label: Text("debug"),
              onPressed: () {
                Future<void> _showMyDialog() async {
                  return showDialog<void>(
                    context: buildContext,
                    barrierDismissible: true, // user must tap button!
                    builder: (BuildContext context) {
                      return AlertDialog(
                        title: const Text('Debug'),
                        content: SingleChildScrollView(
                          child: ListBody(
                            children: <Widget>[
                              Text(
                                debug.value,
                                maxLines: 8,
                              )
                            ],
                          ),
                        ),
                        actions: <Widget>[
                          TextButton(
                            style: TextButton.styleFrom(
                                // primary: Colors.blueGrey[300],
                                backgroundColor: Colors.green,
                                shape: RoundedRectangleBorder(
                                    borderRadius: BorderRadius.circular(10))),
                            child: const Text('Copy',
                                style: TextStyle(
                                  color: Colors.white,
                                )),
                            onPressed: () {
                              _copyToClipboard(debug.value);
                              Navigator.of(context).pop();
                            },
                          ),
                          VerticalDivider(
                            width: 60,
                          ),
                          TextButton(
                            child: const Text('Close',
                                style: TextStyle(color: Colors.blueGrey)),
                            onPressed: () {
                              Navigator.of(context).pop();
                            },
                          ),
                        ],
                      );
                    },
                  );
                }

                store.msgDebug("debug").then((value) {
                  debug.value = value.data!;
                  _showMyDialog();
                });
              },
            ),
            Padding(
              padding: const EdgeInsets.all(8.0),
              child: Divider(color: Colors.white),
            ),
            TextButton.icon(
              style: TextButton.styleFrom(
                primary: Colors.blueGrey.shade400,
              ),
              icon: Icon(Icons.drive_file_rename_outline),
              label: Text("rename graph"),
              onPressed: () {
                Future<void> _showMyDialog() async {
                  return showDialog<void>(
                    context: buildContext,
                    barrierDismissible: true, // user must tap button!
                    builder: (BuildContext context) {
                      return AlertDialog(
                        title: const Text('Rename Graph'),
                        content: SingleChildScrollView(
                          child: ListBody(
                            children: <Widget>[
                              TextField(
                                controller: renameTextEditingController,
                              )
                            ],
                          ),
                        ),
                        actions: <Widget>[
                          TextButton(
                            style: TextButton.styleFrom(
                                // primary: Colors.blueGrey[300],
                                backgroundColor: Colors.green,
                                shape: RoundedRectangleBorder(
                                    borderRadius: BorderRadius.circular(10))),
                            child: const Text('Rename',
                                style: TextStyle(
                                  color: Colors.white,
                                )),
                            onPressed: () {
                              store.msgRenameGraph(graph_entry.id,
                                  renameTextEditingController.text);

                              Navigator.of(context).pop();
                            },
                          ),
                          VerticalDivider(
                            width: 60,
                          ),
                          TextButton(
                            child: const Text('Cancel',
                                style: TextStyle(color: Colors.blueGrey)),
                            onPressed: () {
                              Navigator.of(context).pop();
                            },
                          ),
                        ],
                      );
                    },
                  );
                }

                _showMyDialog();
              },
            ),
            Padding(
              padding: const EdgeInsets.all(8.0),
              child: Divider(color: Colors.white),
            ),
            TextButton.icon(
              style: TextButton.styleFrom(
                primary: Colors.blueGrey.shade400,
              ),
              icon: Icon(Icons.delete_outline),
              label: Text(
                "delete graph",
              ),
              onPressed: () {
                Future<void> _showMyDialog() async {
                  return showDialog<void>(
                    context: buildContext,
                    barrierDismissible: true, // user must tap button!
                    builder: (BuildContext context) {
                      return AlertDialog(
                        title: const Text('Delete Graph'),
                        content: SingleChildScrollView(
                          child: ListBody(
                            children: <Widget>[
                              Text(
                                  "You are about to delete: ${graph_entry.name}"),
                            ],
                          ),
                        ),
                        actions: <Widget>[
                          TextButton(
                            style: TextButton.styleFrom(
                                // primary: Colors.blueGrey[300],
                                backgroundColor: Colors.red,
                                shape: RoundedRectangleBorder(
                                    borderRadius: BorderRadius.circular(10))),
                            child: const Text('DELETE',
                                style: TextStyle(
                                  color: Colors.white,
                                )),
                            onPressed: () {
                              print(graphList.length);
                              if (graphList.length == 0) {
                                store.msgDeleteGraph(graph_entry.id);
                                store.msgLoadGraph("new");
                              }
                              if (graphList.length > 0) {
                                store.msgDeleteGraph(graph_entry.id);
                                final loadGraph = graphList.firstWhere(
                                    (element) => element.id != graph_entry.id);
                                store.msgLoadGraph(loadGraph.id);
                              }
                              Navigator.of(context).pop();
                            },
                          ),
                          VerticalDivider(
                            width: 60,
                          ),
                          TextButton(
                            child: const Text('Cancel',
                                style: TextStyle(color: Colors.blueGrey)),
                            onPressed: () {
                              Navigator.of(context).pop();
                            },
                          ),
                        ],
                      );
                    },
                  );
                }

                _showMyDialog();
              },
            ),
            Container(
              height: 60,
            ),
            Center(
                child: Text(
              "Space Operator, Alpha Version",
              style: TextStyle(color: Colors.white),
            ))
          ],
        ),
      ),
      appBar: AppBar(
        leading: Builder(
          builder: (BuildContext context) {
            return IconButton(
              icon: const Icon(Icons.menu, color: Colors.white),
              onPressed: () {
                Scaffold.of(context).openDrawer();
              },
              tooltip: MaterialLocalizations.of(context).showMenuTooltip,
            );
          },
        ),
        backgroundColor: Colors.black87,
        title: Container(
          child: Row(
            children: [
              Padding(
                padding: const EdgeInsets.fromLTRB(0, 0, 10, 0),
                child: Text("Flow Name:",
                    style: TextStyle(color: Colors.blueGrey.shade400)),
              ),
              DropdownButton(
                  style: TextStyle(
                    color: Colors.amber,
                  ),
                  dropdownColor: Colors.blueGrey.shade900,
                  alignment: AlignmentDirectional.bottomCenter,
                  items: dropDownList,
                  value: graph_entry.id,
                  onChanged: (value) {
                    store.msgLoadGraph(value.toString());
                  }),
            ],
          ),
        ),
        actions: [
          // DebugWidget(),
          TextButton.icon(
              style: TextButton.styleFrom(
                primary: Colors.blueGrey.shade400,
              ),
              icon: Icon(Icons.bookmark_add_outlined),
              onPressed: () {
                if (bookmarkTextController.value.text != "") {
                  ref.read(contextController.notifier).update(buildContext);
                  store
                      .msgCreateBookmark(bookmarkTextController.value.text)
                      .then(
                        (value) => store.msgBookmarkScreenshot(value.data!),
                      );
                  bookmarkTextController.value = TextEditingValue.empty;
                }
              },
              label: Text("Bookmark")),
          Container(
              width: 80,
              child: TextField(
                decoration: InputDecoration(
                  label: Text(
                    "enter name",
                    style: TextStyle(
                        color: Colors.blueGrey.shade400, fontSize: 10),
                  ),
                  // hintText: "bookmark name",
                  // hintStyle: TextStyle(color: Colors.white, fontSize: 10),
                ),
                controller: bookmarkTextController,
                style: TextStyle(color: Colors.blueGrey.shade400),
              )),
          Padding(
            padding: const EdgeInsets.all(8.0),
            child: VerticalDivider(color: Colors.white),
          ),
          Padding(
            padding: const EdgeInsets.all(8.0),
            child: DropdownButton(
                style: TextStyle(
                    fontSize: 18, color: Color.fromARGB(255, 243, 170, 33)),
                alignment: Alignment.bottomCenter,
                items: mainnetList,
                value: mainnetSelection.value,
                onChanged: (value) {
                  mainnetSelection.value = value.toString();
                  store.msgChangeSolanaNet(value.toString());
                }),
          ),
          TextButton.icon(
              style: TextButton.styleFrom(
                primary: Colors.blueGrey.shade400,
              ),
              icon: Icon(Icons.play_arrow_rounded),
              onPressed: () {
                store.msgDeploy("deploy", timeout: Duration(minutes: 120));
              },
              label: Text("Deploy")),
          TextButton.icon(
              style: TextButton.styleFrom(
                primary: Colors.blueGrey.shade400,
              ),
              icon: Icon(Icons.stop),
              onPressed: () {
                store.msgUnDeploy("undeploy");
              },
              label: Text("UnDeploy")),
          Padding(
            padding: const EdgeInsets.all(8.0),
            child: VerticalDivider(color: Colors.white),
          ),
          TextButton.icon(
            style: TextButton.styleFrom(
              primary: Colors.blueGrey.shade400,
            ),
            icon: Icon(Icons.fit_screen),
            label: Text("fit to screen"),
            onPressed: () {
              ref
                  .read(storeRepoProvider)
                  .store
                  .msgFitNodesToScreen("fit to screen");
              // _transformationController.value = Matrix4.identity();
            },
          ),
          TextButton.icon(
            style: TextButton.styleFrom(
              primary: Colors.blueGrey.shade400,
            ),
            icon: Icon(Icons.restart_alt_rounded),
            label: Text("reset zoom"),
            onPressed: () {
              ref.read(storeRepoProvider).store.msgResetZoom("reset zoom");
              // _transformationController.value = Matrix4.identity();
            },
          ),
          IconButton(
            icon: Icon(Icons.zoom_in),
            color: Colors.blueGrey.shade400,
            onPressed: () {
              ref.read(storeRepoProvider).store.msgZoomIn("");
              // _transformationController.value = Matrix4.identity();
            },
          ),
          IconButton(
            icon: Icon(Icons.zoom_out),
            color: Colors.blueGrey.shade400,
            onPressed: () {
              ref.read(storeRepoProvider).store.msgZoomOut("");
              // _transformationController.value = Matrix4.identity();
            },
          ),
        ],
      ),
      body: Stack(children: [
        LayoutBuilder(
          builder: (context, constraints) {
            final resizeEvent = {
              "width": constraints.maxWidth.toInt(), // Screen size
              "height": constraints.maxHeight.toInt(),
            };
            String event = JsonMapper.serialize(InputProperties(resizeEvent));
            ref
                .read(storeRepoProvider)
                .store
                .msgResizeCanvas(event, timeout: Duration(seconds: 20));
            return Container(
              // height: MediaQuery.of(context).size.height + height,
              // width: MediaQuery.of(context).size.width + width,
              decoration: BoxDecoration(color: Colors.blueGrey[900]),
              child: EventListener(),
            );
          },
        ),
        Positioned(
          bottom: 0,
          right: 0,
          child: Image.asset(
            "assets/logo-full-small.png",
            height: 75,
          ),
        ),
        Row(
          children: [
            if (!hideDrawer.value)
              Column(
                children: [
                  const BookmarkManager(),
                  Expanded(
                    child: Container(
                      width: 270,
                      decoration: BoxDecoration(color: Colors.blueGrey[800]),
                      child: ListView.separated(
                        separatorBuilder: ((context, index) => const Divider()),
                        itemBuilder: ((context, index) {
                          return commandList[index];
                        }),
                        itemCount: commandList.length,
                        // children: commandList,
                      ),
                    ),
                  ),
                ],
              ),
            GestureDetector(
              behavior: HitTestBehavior.opaque,
              onTap: () {
                hideDrawer.value = !hideDrawer.value;
              },
              child: Container(
                  width: 30,
                  decoration: BoxDecoration(
                      color: Colors.grey[900],
                      border: Border.all(color: Colors.black26)),
                  child: Column(
                    children: [
                      Expanded(child: Container()),
                      !hideDrawer.value
                          ? Icon(
                              Icons.keyboard_arrow_left_rounded,
                              color: Colors.white,
                            )
                          : Icon(Icons.keyboard_arrow_right_rounded,
                              color: Colors.white),
                      Expanded(child: Container()),
                    ],
                  )),
            )
          ],
        )
      ]),
      // ),
    );
  }
}

class DebugWidget extends HookConsumerWidget {
  const DebugWidget({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final provider = ref.watch(debugController);

    return Container(
      child: Column(children: [
        Text(ref.read(storeRepoProvider).debugData.mappingKind,
            style: TextStyle(color: Colors.white)),
        Text(ref.read(storeRepoProvider).debugData.uiState,
            style: TextStyle(color: Colors.white)),
        Text(ref.read(storeRepoProvider).debugData.selectedNodeIds,
            style: TextStyle(color: Colors.white)),
      ]),
    );
  }
}
