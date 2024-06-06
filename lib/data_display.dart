import 'package:flutter/material.dart';
import 'package:os_lab2/ffi/rust_ffi.dart';

class RecordDisplay extends StatelessWidget {
  const RecordDisplay({
    super.key,
    required this.records,
  });

  final List<MemState> records;

  @override
  Widget build(BuildContext context) {
    const title = ['序号', '指令', '内存块 1', '内存块 2', '内存块 3', '内存块 4', '备注'];
    var headers = List<DataColumn>.generate(
      title.length,
      (int index) => DataColumn(
        label: Expanded(
          flex: 1,
          child: Text(title[index]),
        ),
      ),
    );

    return DataTableTheme(
      data: const DataTableThemeData(
        headingTextStyle: TextStyle(
          fontWeight: FontWeight.bold,
          // fontStyle: FontStyle.italic,
        ),
      ),
      child: PaginatedDataTable(
        rowsPerPage: 20,
        header: const Text("执行结果"),
        columns: headers,
        source: RecordSourceData(
          sourceData: records,
        ),
      ),
    );
  }
}

class RecordSourceData extends DataTableSource {
  RecordSourceData({required this.sourceData});

  final List<MemState> sourceData;

  @override
  bool get isRowCountApproximate => false;

  @override
  int get rowCount => sourceData.length;

  @override
  int get selectedRowCount => 0;

  @override
  DataRow getRow(int index) => DataRow(
    color: WidgetStateProperty.resolveWith(
    (Set<WidgetState> states) {
        if (index.isEven) {
              return Colors.grey.withOpacity(0.2);
            }
        return null;
      }
    ),
    cells: [
      DataCell(
        Row(
          children: [
            const Icon(
              Icons.access_alarm,
              color: Colors.lightGreenAccent,
            ),
            Text('No.${sourceData[index].sequential}'),
          ],
        ),
      ),
      DataCell(
        Row(
          children: [
            const Icon(
              Icons.fork_right,
              color: Colors.cyan,
            ),
            Text('Ins.${sourceData[index].instrument}'),
          ],
        ),
      ),
      DataCell(
        Text('${sourceData[index].frame[0]}'),
      ),
      DataCell(
        Text('${sourceData[index].frame[1]}'),
      ),
      DataCell(
        Text('${sourceData[index].frame[2]}'),
      ),
      DataCell(
        Text('${sourceData[index].frame[3]}'),
      ),
      DataCell(
        Row(
          children: [
            sourceData[index].info.contains('发生缺页')
            ? const Icon(
              Icons.disabled_by_default_rounded,
              color: Colors.red,
            )
            : const Icon(
              Icons.star,
              color: Colors.green,
            ),
            Text(sourceData[index].info),
          ],
        ),
      ),
    ],
  );
}
