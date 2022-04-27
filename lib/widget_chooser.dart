import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
<<<<<<< HEAD
import 'package:moon/nodes/command_widget.dart';
import 'package:moon/providers/store_provider.dart';
import 'package:moon/widgets/text_input.dart';

import 'package:moon/commands/const.dart';

import 'package:moon/nodes/command_widget.dart';
import 'package:moon/commands/print.dart';
import 'package:tuple/tuple.dart';

import './widgets/block.dart';

import 'utils/logger.dart';

SuperBlock WidgetChooser(TreeNode treeNode,
    [nodeType, inputNodes, outputNodes, parentId, ref
    // storedContext,
    ]) {
=======
import 'package:moon/widgets/text_input.dart';
import 'package:moon/widget_input.dart';
import 'package:moon/widget_output.dart';
import 'package:moon/commands/add_pubkey.dart';
import 'package:moon/commands/airdrop.dart';
import 'package:moon/commands/const.dart';
import 'package:moon/commands/create_account.dart';
import 'package:moon/commands/create_token.dart';
import 'package:moon/commands/generate_keypair.dart';
import 'package:moon/nodes/command_widget.dart';
import 'package:moon/commands/print.dart';
import 'package:moon/commands/transfer.dart';

import './widgets/block.dart';

import 'dummy_edge_handle.dart';
import "logger.dart";

HookConsumerWidget WidgetChooser([
  nodeType,
  data,
  children,
  inputNodes,
  outputNodes,
  selectedNode,
  dimensions,
  storedContext,
]) {
>>>>>>> master
  HookConsumerWidget? _widget;

  switch (nodeType) {
    case "WidgetBlock":
      {
        _widget = Block(
<<<<<<< HEAD
          key: ObjectKey(treeNode.node.value),
          treeNode: treeNode,
=======
          node: data,
          children: children,
          // key: ObjectKey(data.nodeKey),
          selected: false,
>>>>>>> master
        );
      }
      break;

    case "WidgetTextInput":
      {
        log.v("adding text input");

        _widget = TextInput(
<<<<<<< HEAD
          key: ObjectKey(treeNode.node.value),
          treeNode: treeNode,
          parentId: parentId,
          // context: storedContext,
=======
          node: data,
          children: children ?? [Text("nochild")],
          // key: ObjectKey(data.nodeKey),
          selected: false,
          // selectedNode: selectedNode,
>>>>>>> master
        );
      }
      break;
    case "DummyEdgeHandle":
<<<<<<< HEAD
    case "WidgetInput":
    case "WidgetOutput":
      {}
=======
      {
        // log.v("adding text input");

        // _widget = DummyEdgeHandle(
        //   node: data,
        //   children: children ?? [Text("nochild")],
        //   // key: ObjectKey(data.nodeKey),
        //   selected: false,
        // );
      }
      break;
    case "WidgetInput":
      {
        // log.v("adding text input");

        // _widget = WidgetInput(
        //   children: children ?? [Text("nochild")],
        //   node: data,
        //   selected: false,
        // );
      }
      break;
    case "WidgetOutput":
      {
        // log.v("adding text input");

        // _widget = WidgetOutput(
        //   children: children ?? [Text("nochild")],
        //   node: data,
        //   selected: false,
        // );
      }
>>>>>>> master
      break;
    //
    case "Const":
      {
<<<<<<< HEAD
        _widget = CommandWidget(
          key: ObjectKey(treeNode.node.value),
          treeNode: treeNode,
          inputs: inputNodes,
          outputs: outputNodes,
          label: nodeType,
          child: Const(
            treeNode: treeNode,
            key: ObjectKey(treeNode.node.value),
          ),
          parentId: parentId,
        );
      }

      break;
    case "Print":
      {
        _widget = CommandWidget(
          key: ObjectKey(treeNode.node.value),
          treeNode: treeNode,
          inputs: inputNodes,
          outputs: outputNodes,
          label: nodeType,
          child: Print(
            treeNode: treeNode,
            key: ObjectKey(treeNode.node.value),
          ),
          parentId: parentId,
        );
      }
      break;
    case "JsonExtract":
    case "JsonInsert":
    case "HttpRequest":
    case "IpfsUpload":
    case "IpfsNftUpload":
    case "Wait":
    case "Branch":
    //
    case "CreateMintAccount":
    case "CreateTokenAccount":
    case "GenerateKeypair":
    case "MintToken":
    case "TransferToken":
    case "TransferSolana":
=======
        _widget = Const(
          node: data,
          key: UniqueKey(),
          selected: false,
          inputs: inputNodes,
          outputs: outputNodes,
        );
      }
      break;
    case "Print":
    case "JsonExtract":
    case "HttpRequest":
    case "IpfsUpload":
    //
    case "CreateToken":
    case "AddPubkey":
    case "CreateAccount":
    case "GenerateKeypair":
    case "MintToken":
    case "Transfer":
>>>>>>> master
    case "RequestAirdrop":
    case "GetBalance":
    //
    case "CreateMetadataAccounts":
    case "CreateMasterEdition":
    case "UpdateMetadataAccounts":
<<<<<<< HEAD
    case "VerifyCollection":
    case "ApproveCollectionAuthority":
    case "SignMetadata":
    case "Utilize":
    case "ApproveUseAuthority":
    case "GetLeftUses":
    // case "ArweaveNftUpload":
    // case "ArweaveUpload":
    case "ArweaveFileUpload":
    case "ArweaveNftUpload":
      {
        _widget = CommandWidget(
          key: ObjectKey(treeNode.node.value),
          treeNode: treeNode,
          inputs: inputNodes,
          outputs: outputNodes,
          label: nodeType,
          parentId: parentId,
=======
    case "Utilize":
    case "ApproveUseAuthority":
    case "GetLeftUses":
    case "ArweaveUpload":
      {
        _widget = CommandWidget(
          node: data,
          key: UniqueKey(),
          selected: false,
          inputs: inputNodes,
          outputs: outputNodes,
          label: nodeType,
>>>>>>> master
        );
      }
      break;
    default:
      {
        // return Text("No Matching cases: ${data.properties}");
      }
  }

<<<<<<< HEAD
  return _widget as SuperBlock;
}
// }
// CommandWidget(
//               key: UniqueKey(),
//               treeNode: treeNode,
//               inputs: inputNodes,
//               outputs: outputNodes,
//               label: nodeType,
//               parentId: parentId,
//             );

// final commandProvider = Provider.family<CommandWidget, Tuple5>(
//   (ref, tuple) {
//     return CommandWidget(
//       key: UniqueKey(),
//       treeNode: tuple.item1,
//       inputs: tuple.item2,
//       outputs: tuple.item3,
//       label: tuple.item4,
//       parentId: tuple.item5,
//     );
//   },
// );
=======
  return _widget as HookConsumerWidget;
}
// }
>>>>>>> master
