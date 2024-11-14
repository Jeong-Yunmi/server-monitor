// flutter_client/lib/core/di/service_locator.dart
import 'package:get_it/get_it.dart';
import 'package:shared_preferences/shared_preferences.dart';
import '../api/api_client.dart';
import '../theme/theme_provider.dart';

final getIt = GetIt.instance;

Future<void> setupServiceLocator() async {
  // SharedPreferences 초기화
  final prefs = await SharedPreferences.getInstance();
  getIt.registerSingleton(prefs);

  // Providers
  getIt.registerSingleton(ThemeProvider(getIt<SharedPreferences>()));

  // API Client
  getIt.registerSingleton(ApiClient(getIt<SharedPreferences>()));
}
