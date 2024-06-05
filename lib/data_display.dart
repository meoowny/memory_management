import 'package:flutter/material.dart';
import 'package:os_lab2/ffi/rust_ffi.dart';

class RecordDisplay extends StatelessWidget {
  const RecordDisplay({
    super.key,
    required this.record,
  });

  final ExecRecord record;

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

    var tableContents = List<DataRow>.generate(
      record.records.length,
      (int index) => DataRow(
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
                const Icon(Icons.access_alarm),
                Text('No.${record.records[index].sequential}'),
              ],
            ),
          ),
          DataCell(
            Row(
              children: [
                const Icon(Icons.fork_right),
                Text('Ins.${record.records[index].instrument}'),
              ],
            ),
          ),
          DataCell(
            Text('${record.records[index].frame[0]}'),
          ),
          DataCell(
            Text('${record.records[index].frame[1]}'),
          ),
          DataCell(
            Text('${record.records[index].frame[2]}'),
          ),
          DataCell(
            Text('${record.records[index].frame[3]}'),
          ),
          DataCell(
            Row(
              children: [
                record.records[index].info.contains('发生缺页')
                  ? const Icon(Icons.disabled_by_default_rounded)
                  : const Icon(Icons.star),
                Text(record.records[index].info),
              ],
            ),
          ),
        ],
      ),
    );

    return SingleChildScrollView(
      scrollDirection: Axis.horizontal,
      child: DataTable(
        border: TableBorder.all(),
        headingTextStyle: const TextStyle(
          fontWeight: FontWeight.bold,
          // fontStyle: FontStyle.italic,
        ),
        columns: headers,
        rows: tableContents,
      )
    );
  }
}
