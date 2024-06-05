import 'package:flutter/material.dart';
import 'package:os_lab2/ffi/rust_ffi.dart';
import 'package:os_lab2/ffi/native_ffi.dart';
import 'package:sticky_table/flutter_sticky_table.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Flutter Demo',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.deepPurple),
        useMaterial3: true,
      ),
      home: const MyHomePage(title: 'Flutter Demo Home Page'),
    );
  }
}

class MyHomePage extends StatefulWidget {
  const MyHomePage({super.key, required this.title});

  final String title;

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  int _counter = 0;
  ExecRecord _record = ExecRecord(records: [], totalInstrument: 320, totalFaults: 0);

  void _incrementCounter() {
    // https://api.flutter.dev/flutter/material/DataTable-class.html
    setState(() async {
      _counter++;
      _record = await NativeFun.generateReplacementRecord(320, Choice.LRU);
    });
  }

  Widget displayRecords() {
    List<DataRow> tbl = [];
    for (var i in _record.records) {
      tbl.add(DataRow(
        cells: [
          DataCell(Text(i.frame[0].toString())),
          DataCell(Text(i.frame[1].toString())),
          DataCell(Text(i.frame[2].toString())),
        ],
      ));
    }
    return DataTable(
      columns: const <DataColumn>[
        DataColumn(
          label: Expanded(
            child: Text(
              'Name',
              style: TextStyle(fontStyle: FontStyle.italic),
            ),
          ),
        ),
        DataColumn(
          label: Expanded(
            child: Text(
              'Age',
              style: TextStyle(fontStyle: FontStyle.italic),
            ),
          ),
        ),
        DataColumn(
          label: Expanded(
            child: Text(
              'Role',
              style: TextStyle(fontStyle: FontStyle.italic),
            ),
          ),
        ),
      ],
      rows: tbl,
    );
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        backgroundColor: Theme.of(context).colorScheme.inversePrimary,
        title: Text(widget.title),
      ),
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: <Widget>[
            const Text(
              'You have pushed the button this many times:',
            ),
            Text(
              '$_counter',
              style: Theme.of(context).textTheme.headlineMedium,
            ),
            displayRecords(),
            // Text(_record.records[0].info),
          ],
        ),
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: _incrementCounter,
        tooltip: 'Increment',
        child: const Icon(Icons.add),
      ),
    );
  }
}
