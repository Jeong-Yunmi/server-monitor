// flutter_client/lib/features/dashboard/dashboard_provider.dart
import 'dart:async';
import 'package:flutter/material.dart';
import '../../shared/models/server.dart';
import '../../shared/models/server_metrics.dart';
import '../../shared/models/process_info.dart';
import 'models/dashboard_stats.dart';

class DashboardProvider extends ChangeNotifier {
  bool _isLoading = false;
  String? _error;
  List<Server> _servers = [];
  Map<String, ServerMetrics> _metrics = {};

  bool get isLoading => _isLoading;
  String? get error => _error;
  List<Server> get servers => _servers;
  Map<String, ServerMetrics> get metrics => _metrics;

  DashboardProvider() {
    initializeMonitoring();
  }

  Future<void> initializeMonitoring() async {
    _isLoading = true;
    _error = null;
    notifyListeners();

    try {
      // 개발용 더미 데이터
      await Future.delayed(const Duration(seconds: 1));
      _servers = [
        Server(
          id: '1',
          name: 'Production Server',
          isOnline: true,
          type: 'Production',
          location: 'US-East',
          metrics: const ServerMetrics(
            cpu: 45.0,
            memory: 60.0,
            disk: 75.0,
            network: 30.0,
            uptime: '15d 7h',
            processes: [
              ProcessInfo(
                name: 'nginx',
                cpu: 2.5,
                memory: 1.8,
                network: '150MB/s',
                pid: 1234,
              ),
              ProcessInfo(
                name: 'mongodb',
                cpu: 4.2,
                memory: 3.1,
                network: '80MB/s',
                pid: 1235,
              ),
            ],
          ),
        ),
        Server(
          id: '2',
          name: 'Development Server',
          isOnline: true,
          type: 'Development',
          location: 'US-West',
          metrics: const ServerMetrics(
            cpu: 30.0,
            memory: 40.0,
            disk: 50.0,
            network: 25.0,
            uptime: '7d 12h',
            processes: [
              ProcessInfo(
                name: 'nodejs',
                cpu: 1.5,
                memory: 1.2,
                network: '50MB/s',
                pid: 1236,
              ),
              ProcessInfo(
                name: 'redis',
                cpu: 1.0,
                memory: 0.8,
                network: '20MB/s',
                pid: 1237,
              ),
            ],
          ),
        ),
      ];

      // 메트릭스 데이터 초기화
      for (final server in _servers) {
        _metrics[server.id] = server.metrics;
      }

      // 메트릭스 주기적 업데이트 시뮬레이션
      _startMetricsSimulation();
    } catch (e) {
      _error = 'Failed to initialize monitoring: $e';
    } finally {
      _isLoading = false;
      notifyListeners();
    }
  }

  void _startMetricsSimulation() {
    Timer.periodic(const Duration(seconds: 3), (timer) {
      for (final serverId in _metrics.keys) {
        final currentMetrics = _metrics[serverId]!;

        // 랜덤한 변동 시뮬레이션
        _metrics[serverId] = ServerMetrics(
          cpu: _updateMetric(currentMetrics.cpu),
          memory: _updateMetric(currentMetrics.memory),
          disk: _updateMetric(currentMetrics.disk),
          network: _updateMetric(currentMetrics.network),
          uptime: currentMetrics.uptime,
          processes: currentMetrics.processes,
        );
      }
      notifyListeners();
    });
  }

  double _updateMetric(double currentValue) {
    // -5에서 +5 사이의 랜덤한 변동
    final change = (DateTime.now().millisecond % 10 - 5) / 2;
    return (currentValue + change).clamp(0, 100);
  }

  DashboardStats getStats() {
    if (_servers.isEmpty) {
      return DashboardStats.empty();
    }

    final activeServers = _servers.where((s) => s.isOnline).length;
    final warningServers = _metrics.values
        .where((m) => m.cpu > 80 || m.memory > 80 || m.disk > 80)
        .length;
    final criticalServers = _metrics.values
        .where((m) => m.cpu > 90 || m.memory > 90 || m.disk > 90)
        .length;

    return DashboardStats(
      totalServers: _servers.length,
      activeServers: activeServers,
      warningServers: warningServers,
      criticalServers: criticalServers,
      averageCpu: _calculateAverage((s) => s.metrics.cpu),
      averageMemory: _calculateAverage((s) => s.metrics.memory),
      averageDisk: _calculateAverage((s) => s.metrics.disk),
    );
  }

  double _calculateAverage(double Function(Server) selector) {
    if (_servers.isEmpty) return 0;
    return _servers.map(selector).reduce((a, b) => a + b) / _servers.length;
  }
}
