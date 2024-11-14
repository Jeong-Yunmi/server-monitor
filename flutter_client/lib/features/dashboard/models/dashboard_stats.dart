// flutter_client/lib/features/dashboard/models/dashboard_stats.dart
class DashboardStats {
  final int totalServers;
  final int activeServers;
  final int warningServers;
  final int criticalServers;
  final double averageCpu;
  final double averageMemory;
  final double averageDisk;

  const DashboardStats({
    required this.totalServers,
    required this.activeServers,
    required this.warningServers,
    required this.criticalServers,
    required this.averageCpu,
    required this.averageMemory,
    required this.averageDisk,
  });

  factory DashboardStats.empty() => const DashboardStats(
        totalServers: 0,
        activeServers: 0,
        warningServers: 0,
        criticalServers: 0,
        averageCpu: 0,
        averageMemory: 0,
        averageDisk: 0,
      );
}
