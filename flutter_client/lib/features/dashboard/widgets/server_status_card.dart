// flutter_client/lib/features/dashboard/widgets/server_status_card.dart
import 'package:flutter/material.dart';
import '../../../shared/models/server.dart';
import '../../../shared/models/server_metrics.dart';

class ServerStatusCard extends StatelessWidget {
  final List<Server> servers;
  final Map<String, ServerMetrics> metrics;

  const ServerStatusCard({
    super.key,
    required this.servers,
    required this.metrics,
  });

  @override
  Widget build(BuildContext context) {
    return Card(
      child: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                Text(
                  'Server Status',
                  style: Theme.of(context).textTheme.titleLarge?.copyWith(
                        fontWeight: FontWeight.bold,
                      ),
                ),
                TextButton(
                  onPressed: () {
                    // TODO: Navigate to server list
                  },
                  child: const Text('See All'),
                ),
              ],
            ),
            const SizedBox(height: 16),
            ListView.builder(
              shrinkWrap: true,
              physics: const NeverScrollableScrollPhysics(),
              itemCount: servers.length.clamp(0, 4), // Show max 4 servers
              itemBuilder: (context, index) {
                final server = servers[index];
                final serverMetrics = metrics[server.id];
                return _ServerStatusItem(
                  server: server,
                  metrics: serverMetrics,
                );
              },
            ),
          ],
        ),
      ),
    );
  }
}

class _ServerStatusItem extends StatelessWidget {
  final Server server;
  final ServerMetrics? metrics;

  const _ServerStatusItem({
    required this.server,
    this.metrics,
  });

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.only(bottom: 12.0),
      child: Row(
        children: [
          Icon(
            Icons.circle,
            size: 12,
            color: server.isOnline ? Colors.green : Colors.red,
          ),
          const SizedBox(width: 8),
          Expanded(
            flex: 2,
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  server.name,
                  style: const TextStyle(
                    fontWeight: FontWeight.w500,
                  ),
                ),
                Text(
                  server.location,
                  style: TextStyle(
                    color: Colors.grey[400],
                    fontSize: 12,
                  ),
                ),
              ],
            ),
          ),
          if (metrics != null) ...[
            Expanded(
              child: _buildMetric(
                'CPU',
                metrics!.cpu,
                Theme.of(context).primaryColor,
              ),
            ),
            Expanded(
              child: _buildMetric(
                'RAM',
                metrics!.memory,
                Colors.blue,
              ),
            ),
            Expanded(
              child: _buildMetric(
                'Disk',
                metrics!.disk,
                Colors.green,
              ),
            ),
          ],
        ],
      ),
    );
  }

  Widget _buildMetric(String label, double value, Color color) {
    return Column(
      children: [
        Text(
          '${value.toStringAsFixed(1)}%',
          style: TextStyle(
            color: color,
            fontWeight: FontWeight.w500,
          ),
        ),
        Text(
          label,
          style: TextStyle(
            color: Colors.grey[400],
            fontSize: 12,
          ),
        ),
      ],
    );
  }
}
