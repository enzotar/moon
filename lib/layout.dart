import 'dart:typed_data';

import 'package:dart_json_mapper/dart_json_mapper.dart';
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

    return Scaffold(
      key: _scaffoldKey,
      drawer: Drawer(
        backgroundColor: Colors.blueGrey[900],
        child: ListView(
          children: [
            TextButton.icon(
              icon: Icon(Icons.mouse),
              label: Text("Set Input For Mouse"),
              onPressed: () {
                ref.read(storeRepoProvider).store.msgSetMappingKind("mouse");
              },
            ),
            TextButton.icon(
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
              icon: Icon(Icons.save),
              label: Text("export"),
              onPressed: () async {
                final timestamp =
                    DateTime.now().millisecondsSinceEpoch.toString();
                final filename = ref.read(storeRepoProvider).graph_entry.name +
                    " - " +
                    timestamp; // TODO fix datetime format
                print(filename);
                String? path = await FilePicker.platform.saveFile(
                    fileName: filename,
                    type: FileType.custom,
                    allowedExtensions: ["json"]);

                if (path != null) {
                  print(path);
                  store.msgExport(path, filename);
                }
              },
            ),
            Padding(
              padding: const EdgeInsets.all(8.0),
              child: Divider(color: Colors.white),
            ),
            TextButton.icon(
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
            TextButton(
              child: Text("debug"),
              onPressed: () {
                store.msgDebug("debug");
              },
            ),
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
                child: Text("Flow Name:", style: TextStyle(color: Colors.blue)),
              ),
              DropdownButton(
                  style: TextStyle(color: Colors.blue),
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
          DebugWidget(),
          TextButton.icon(
              icon: Icon(Icons.bookmark_add_outlined),
              onPressed: () {
                ref.read(contextController.notifier).update(buildContext);

                store.msgCreateBookmark("add_bookmark").then(
                      (value) => store.msgBookmarkScreenshot(value.data!),
                    );
              },
              label: Text("Bookmark Nodes")),
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
              icon: Icon(Icons.play_arrow_rounded),
              onPressed: () {
                store.msgDeploy("deploy", timeout: Duration(minutes: 120));
              },
              label: Text("Deploy")),
          TextButton.icon(
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
            icon: Icon(Icons.restart_alt_rounded),
            label: Text("reset zoom"),
            onPressed: () {
              ref.read(storeRepoProvider).store.msgResetZoom("reset zoom");
              // _transformationController.value = Matrix4.identity();
            },
          ),
          TextButton.icon(
            icon: Icon(Icons.zoom_in),
            label: Text("+"),
            onPressed: () {
              ref.read(storeRepoProvider).store.msgZoomIn("");
              // _transformationController.value = Matrix4.identity();
            },
          ),
          TextButton.icon(
            icon: Icon(Icons.zoom_out),
            label: Text("-"),
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
        // Image.asset(
        //   "assets/logo-full-small.png",
        //   height: 100,
        // ),

        Row(
          children: [
            if (!hideDrawer.value) const BookmarkManager(),
            // Container(
            //   decoration: BoxDecoration(color: Colors.blueGrey[800]),
            //   child: ListView.separated(
            //     separatorBuilder: ((context, index) => const Divider()),
            //     itemBuilder: ((context, index) {
            //       return commandList[index];
            //     }),
            //     itemCount: commandList.length,
            //     // children: commandList,
            //   ),
            // ),
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
