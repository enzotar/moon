import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:plugin/generated/rid_api.dart';
import 'package:moon/layout.dart';
import 'package:dart_json_mapper/dart_json_mapper.dart';

import 'package:moon/rid/messaging.dart';
import 'package:moon/serialization/input_mapping.dart';
import 'package:moon/serialization/main.mapper.g.dart';
import 'package:path_provider/path_provider.dart';

void main() async {
  RidMessaging.init();

  rid.debugLock = null;
  rid.debugReply = null; // rid.debugReply = (reply) => debugPrint('$reply');

  WidgetsFlutterBinding.ensureInitialized();
  final appDir = await getApplicationSupportDirectory();
  await Store.instance.msgInitialize(appDir.path);
  initializeJsonMapper();

  // rid_ffi.rid_export_query_graph();
  // rid_ffi.rid_export_start();
  // rid_ffi.rid_export_handle_input2(1);

  runApp(ProviderScope(child: MyApp()));
  print(
    JsonMapper.serialize(
      InputProperties({"propertyName": "property"}),
    ),
  );
}

class MyApp extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
        debugShowCheckedModeBanner: false,
        title: 'Dragonfly',
        theme: ThemeData(
          primarySwatch: Colors.blue,
          visualDensity: VisualDensity.adaptivePlatformDensity,
        ),
        home: LayoutScreen());
  }
}
