import 'package:flutter/material.dart';
import 'package:os_lab2/ffi/rust_ffi.dart';
import 'package:os_lab2/ffi/native_ffi.dart';
import 'package:os_lab2/data_display.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatefulWidget {
  const MyApp({super.key});

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  ThemeMode themeMode = ThemeMode.system;

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'OS Lab2',
      theme: ThemeData(
        colorScheme: const ColorScheme.light(),
        // colorScheme: ColorScheme.fromSeed(seedColor: Colors.lightBlue),
        // brightness: ThemeMode.system == ThemeMode.dark ? Brightness.dark : Brightness.light,
        useMaterial3: true,
        fontFamily: 'SourceHanSansSC',
      ),
      darkTheme: ThemeData(
        colorScheme: const ColorScheme.dark(),
        useMaterial3: true,
        fontFamily: 'SourceHanSansSC',
      ),
      themeMode: themeMode,
      home: const MyHomePage(title: 'OS Lab2: Memory Management'),
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
  AlgoChoice _algoChoice = AlgoChoice.FIFO;
  GenChoice _genChoice = GenChoice.Random;
  ExecRecord _record = const ExecRecord(records: [], totalInstrument: 320, totalFaults: 0);

  void _incrementCounter() async {
    _record = await NativeFun.generateReplacementRecord(_algoChoice, _genChoice);
    setState(() {});
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        backgroundColor: Theme.of(context).colorScheme.inversePrimary,
        title: Text(widget.title),
      ),
      body: ListView(
        children: <Widget>[
          Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              Container(height: 20),
              SizedBox(
                width: 500,
                child: Row(
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: [
                    Expanded(
                      child: AlgorithmRadio(
                        choice: _algoChoice,
                        onChanged: (AlgoChoice? value) {
                          setState(() {
                            _algoChoice = value!;
                          });
                        },
                      ),
                    ),
                    Expanded(
                      child: GeneratorRadio(
                        choice: _genChoice,
                        onChanged: (GenChoice? value) {
                          setState(() {
                            _genChoice = value!;
                          });
                        },
                      ),
                    ),
                  ],
                ),
              ),
              Row(
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                  Expanded(
                    flex: 1,
                    child: Center(
                      child: _StatisticsInfoCard(content: "总指令条数：${_record.totalInstrument}"),
                    ),
                  ),
                  Expanded(
                    flex: 1,
                    child: Center(
                      child: _StatisticsInfoCard(content: "总缺页数：${_record.totalFaults}"),
                    ),
                  ),
                  Expanded(
                    flex: 1,
                    child: Center(
                      child: _StatisticsInfoCard(content: "总缺页率：${(_record.totalFaults / _record.totalInstrument * 100).toStringAsFixed(2)}%"),
                    ),
                  ),
                ],
              ),
              Container(
                margin: const EdgeInsets.only(bottom: 20),
                child: FilledButton(onPressed: _incrementCounter, child: const Text('开始执行')),
              ),
            ]
          ),
          SizedBox(
            child: RecordDisplay(records: _record.records),
          ),
        ],
      ),
    );
  }
}

class _StatisticsInfoCard extends StatelessWidget {
  const _StatisticsInfoCard({required this.content});
  final String content;

  @override
  Widget build(BuildContext context)  {
    return Card(
      color: Theme.of(context).cardColor.withAlpha(100),
      margin: const EdgeInsets.all(20),
      child: SizedBox(
        width: 200,
        height: 80,
        child: Center(
          child: Text(
            content,
            style: Theme.of(context).textTheme.bodyLarge,
          ),
        ),
      ),
    );
  }
}

class AlgorithmRadio extends StatelessWidget {
  const AlgorithmRadio({
    super.key,
    this.choice = AlgoChoice.LRU,
    required this.onChanged,
  });

  final AlgoChoice choice;
  final ValueChanged<AlgoChoice?> onChanged;

  @override
  Widget build(BuildContext context) {
    return Column(
      mainAxisAlignment: MainAxisAlignment.center,
      children: [
        const Text('选用置换算法'),
        ListTile(
          title: const Text("FIFO"),
          leading: Radio<AlgoChoice>(
          value: AlgoChoice.FIFO,
          groupValue: choice,
          onChanged: onChanged,
        ),
        ),
        ListTile(
          title: const Text("LRU"),
          leading: Radio<AlgoChoice>(
          value: AlgoChoice.LRU,
          groupValue: choice,
          onChanged: onChanged,
        ),
        ),
      ],
    );
  }
}

class GeneratorRadio extends StatelessWidget {
  const GeneratorRadio({
    super.key,
    this.choice = GenChoice.Random,
    required this.onChanged,
  });

  final GenChoice choice;
  final ValueChanged<GenChoice?> onChanged;

  @override
  Widget build(BuildContext context) {
    return Column(
      mainAxisAlignment: MainAxisAlignment.center,
      children: [
        const Text('选用指令序列生成方法'),
        ListTile(
          title: const Text("Random"),
          style: Theme.of(context).listTileTheme.style,
          leading: Radio<GenChoice>(
            value: GenChoice.Random,
            groupValue: choice,
            onChanged: onChanged,
          ),
        ),
        ListTile(
          title: const Text("Specific"),
          leading: Radio<GenChoice>(
          value: GenChoice.Sequential,
          groupValue: choice,
          onChanged: onChanged,
        ),
        ),
      ],
    );
  }
}
