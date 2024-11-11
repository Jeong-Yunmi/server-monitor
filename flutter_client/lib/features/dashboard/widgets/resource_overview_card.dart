// lib/features/dashboard/widgets/resource_overview_card.dart

import 'package:flutter/material.dart';
import 'package:fl_chart/fl_chart.dart';

class ResourceOverviewCard extends StatelessWidget {
  const ResourceOverviewCard({Key? key}) : super(key: key);

  Widget leftTitleWidgets(double value, TitleMeta meta) {
    return Text(
      '${value.toInt()}%',
      style: TextStyle(
        color: Colors.grey[400],
        fontSize: 12,
      ),
    );
  }

  Widget bottomTitleWidgets(double value, TitleMeta meta) {
    const times = ['12:00', '13:00', '14:00', '15:00', '16:00', '17:00'];
    if (value.toInt() < times.length) {
      return Text(
        times[value.toInt()],
        style: TextStyle(
          color: Colors.grey[400],
          fontSize: 12,
        ),
      );
    }
    return const Text('');
  }

  @override
  Widget build(BuildContext context) {
    return Card(
      color: Colors.grey[900],
      child: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
              'Resource Overview',
              style: Theme.of(context).textTheme.titleLarge?.copyWith(
                    fontWeight: FontWeight.bold,
                  ),
            ),
            const SizedBox(height: 24),
            SizedBox(
              height: 200,
              child: LineChart(
                LineChartData(
                  gridData: FlGridData(show: true),
                  titlesData: FlTitlesData(
                    rightTitles: const AxisTitles(
                        sideTitles: SideTitles(showTitles: false)),
                    topTitles: const AxisTitles(
                        sideTitles: SideTitles(showTitles: false)),
                    leftTitles: AxisTitles(
                      sideTitles: SideTitles(
                        showTitles: true,
                        getTitlesWidget: leftTitleWidgets,
                      ),
                    ),
                    bottomTitles: AxisTitles(
                      sideTitles: SideTitles(
                        showTitles: true,
                        getTitlesWidget: bottomTitleWidgets,
                      ),
                    ),
                  ),
                  borderData: FlBorderData(show: false),
                  lineBarsData: [
                    // CPU Line
                    LineChartBarData(
                      spots: [
                        const FlSpot(0, 30),
                        const FlSpot(1, 45),
                        const FlSpot(2, 35),
                        const FlSpot(3, 55),
                        const FlSpot(4, 40),
                        const FlSpot(5, 45),
                      ],
                      isCurved: true,
                      color: Theme.of(context).primaryColor,
                      barWidth: 2,
                      dotData: FlDotData(show: false),
                    ),
                    // Memory Line
                    LineChartBarData(
                      spots: [
                        const FlSpot(0, 50),
                        const FlSpot(1, 55),
                        const FlSpot(2, 45),
                        const FlSpot(3, 65),
                        const FlSpot(4, 50),
                        const FlSpot(5, 55),
                      ],
                      isCurved: true,
                      color: Colors.blue,
                      barWidth: 2,
                      dotData: FlDotData(show: false),
                    ),
                    // Network Line
                    LineChartBarData(
                      spots: [
                        const FlSpot(0, 20),
                        const FlSpot(1, 25),
                        const FlSpot(2, 15),
                        const FlSpot(3, 35),
                        const FlSpot(4, 20),
                        const FlSpot(5, 25),
                      ],
                      isCurved: true,
                      color: Colors.green,
                      barWidth: 2,
                      dotData: FlDotData(show: false),
                    ),
                  ],
                ),
              ),
            ),
            const SizedBox(height: 16),
            Row(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                _buildLegendItem('CPU', Theme.of(context).primaryColor),
                const SizedBox(width: 24),
                _buildLegendItem('Memory', Colors.blue),
                const SizedBox(width: 24),
                _buildLegendItem('Network', Colors.green),
              ],
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildLegendItem(String label, Color color) {
    return Row(
      children: [
        Container(
          width: 12,
          height: 12,
          decoration: BoxDecoration(
            color: color,
            borderRadius: BorderRadius.circular(3),
          ),
        ),
        const SizedBox(width: 8),
        Text(
          label,
          style: const TextStyle(
            color: Colors.white70,
            fontSize: 12,
          ),
        ),
      ],
    );
  }
}
