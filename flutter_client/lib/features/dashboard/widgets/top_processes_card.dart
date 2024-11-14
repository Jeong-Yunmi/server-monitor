// flutter_client/lib/features/dashboard/widgets/top_processes_card.dart
import 'package:flutter/material.dart';
import '../../../shared/models/process_info.dart';

class TopProcessesCard extends StatelessWidget {
  final List<ProcessInfo> processes;

  const TopProcessesCard({
    super.key,
    required this.processes,
  });

  @override
  Widget build(BuildContext context) {
    // Sort processes by CPU usage
    final sortedProcesses = [...processes]
      ..sort((a, b) => b.cpu.compareTo(a.cpu));

    return Card(
      child: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
              'Top Processes',
              style: Theme.of(context).textTheme.titleLarge?.copyWith(
                    fontWeight: FontWeight.bold,
                  ),
            ),
            const SizedBox(height: 16),
            Table(
              columnWidths: const {
                0: FlexColumnWidth(2.5),
                1: FlexColumnWidth(1.2),
                2: FlexColumnWidth(1.2),
                3: FlexColumnWidth(1.5),
              },
              children: [
                TableRow(
                  children: [
                    _buildTableHeader('PROCESS'),
                    _buildTableHeader('CPU (%)'),
                    _buildTableHeader('RAM (GB)'),
                    _buildTableHeader('NETWORK'),
                  ],
                ),
                ...sortedProcesses.take(4).map((process) => _buildTableRow(
                      process: process.name,
                      cpu: process.cpu,
                      ram: process.memory,
                      network: process.network,
                      pid: process.pid,
                    )),
              ],
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildTableHeader(String text) {
    return Padding(
      padding: const EdgeInsets.only(bottom: 12),
      child: Text(
        text,
        style: const TextStyle(
          color: Colors.grey,
          fontSize: 12,
          fontWeight: FontWeight.w600,
        ),
      ),
    );
  }

  TableRow _buildTableRow({
    required String process,
    required double cpu,
    required double ram,
    required String network,
    required int pid,
  }) {
    return TableRow(
      children: [
        _buildTableCell(
          '$process (PID: $pid)',
          color: Colors.white,
          fontWeight: FontWeight.w500,
        ),
        _buildTableCell('${cpu.toStringAsFixed(1)}%'),
        _buildTableCell(ram.toStringAsFixed(1)),
        _buildTableCell(network),
      ],
    );
  }

  Widget _buildTableCell(
    String text, {
    Color color = Colors.grey,
    FontWeight fontWeight = FontWeight.normal,
  }) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 8),
      child: Text(
        text,
        style: TextStyle(
          color: color,
          fontSize: 13,
          fontWeight: fontWeight,
        ),
      ),
    );
  }
}
