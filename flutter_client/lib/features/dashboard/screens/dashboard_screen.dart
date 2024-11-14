// flutter_client/lib/features/dashboard/screens/dashboard_screen.dart
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import '../dashboard_provider.dart';
import '../widgets/resource_overview_card.dart';
import '../widgets/server_status_card.dart';
import '../widgets/distribution_card.dart';
import '../widgets/top_processes_card.dart';
import '../models/dashboard_stats.dart';

class DashboardScreen extends StatelessWidget {
  const DashboardScreen({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text(
          'Dashboard',
          style: TextStyle(fontWeight: FontWeight.bold),
        ),
        actions: [
          IconButton(
            icon: const Icon(Icons.notifications_none),
            onPressed: () {
              // TODO: Navigate to notifications
            },
          ),
          IconButton(
            icon: const Icon(Icons.settings_outlined),
            onPressed: () {
              // TODO: Navigate to settings
            },
          ),
        ],
      ),
      body: Consumer<DashboardProvider>(
        builder: (context, provider, child) {
          if (provider.isLoading) {
            return const _LoadingView();
          }

          if (provider.error != null) {
            return _ErrorView(
              message: provider.error!,
              onRetry: () => provider.initializeMonitoring(), // 변경된 부분
            );
          }

          final stats = provider.getStats();

          return RefreshIndicator(
            onRefresh: () => provider.initializeMonitoring(), // 변경된 부분
            child: SingleChildScrollView(
              physics: const AlwaysScrollableScrollPhysics(),
              padding: const EdgeInsets.all(16.0),
              child: Column(
                children: [
                  _buildStatusOverview(stats),
                  const SizedBox(height: 16),
                  ResourceOverviewCard(
                    metrics: provider.metrics,
                    stats: stats,
                  ),
                  const SizedBox(height: 16),
                  ServerStatusCard(
                    servers: provider.servers,
                    metrics: provider.metrics,
                  ),
                  const SizedBox(height: 16),
                  TopProcessesCard(
                    processes: provider.metrics.values
                        .expand((m) => m.processes)
                        .toList(),
                  ),
                  const SizedBox(height: 16),
                  DistributionCard(metrics: provider.metrics),
                  const SizedBox(height: 32),
                ],
              ),
            ),
          );
        },
      ),
    );
  }

  Widget _buildStatusOverview(DashboardStats stats) {
    return Row(
      children: [
        _StatusCard(
          title: 'Total Servers',
          value: stats.totalServers.toString(),
          icon: Icons.computer,
          color: Colors.blue,
        ),
        _StatusCard(
          title: 'Active',
          value: stats.activeServers.toString(),
          icon: Icons.check_circle,
          color: Colors.green,
        ),
        _StatusCard(
          title: 'Warnings',
          value: stats.warningServers.toString(),
          icon: Icons.warning,
          color: Colors.orange,
        ),
        _StatusCard(
          title: 'Critical',
          value: stats.criticalServers.toString(),
          icon: Icons.error,
          color: Colors.red,
        ),
      ],
    );
  }
}

class _StatusCard extends StatelessWidget {
  final String title;
  final String value;
  final IconData icon;
  final Color color;

  const _StatusCard({
    required this.title,
    required this.value,
    required this.icon,
    required this.color,
  });

  @override
  Widget build(BuildContext context) {
    return Expanded(
      child: Card(
        child: Padding(
          padding: const EdgeInsets.all(12.0),
          child: Column(
            children: [
              Icon(icon, color: color, size: 24),
              const SizedBox(height: 8),
              Text(
                value,
                style: TextStyle(
                  fontSize: 20,
                  fontWeight: FontWeight.bold,
                  color: color,
                ),
              ),
              const SizedBox(height: 4),
              Text(
                title,
                style: const TextStyle(
                  fontSize: 12,
                  color: Colors.grey,
                ),
                textAlign: TextAlign.center,
              ),
            ],
          ),
        ),
      ),
    );
  }
}

class _LoadingView extends StatelessWidget {
  const _LoadingView();

  @override
  Widget build(BuildContext context) {
    return const Center(
      child: CircularProgressIndicator(),
    );
  }
}

class _ErrorView extends StatelessWidget {
  final String message;
  final VoidCallback onRetry;

  const _ErrorView({
    required this.message,
    required this.onRetry,
  });

  @override
  Widget build(BuildContext context) {
    return Center(
      child: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            const Icon(
              Icons.error_outline,
              size: 48,
              color: Colors.red,
            ),
            const SizedBox(height: 16),
            Text(
              message,
              textAlign: TextAlign.center,
              style: const TextStyle(color: Colors.grey),
            ),
            const SizedBox(height: 16),
            ElevatedButton(
              onPressed: onRetry,
              child: const Text('Retry'),
            ),
          ],
        ),
      ),
    );
  }
}
