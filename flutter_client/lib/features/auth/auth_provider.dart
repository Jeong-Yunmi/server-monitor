// flutter_client/lib/features/auth/auth_provider.dart
import 'package:flutter/material.dart';
import 'package:shared_preferences/shared_preferences.dart';
//import '../../core/api/api_client.dart';
import '../../core/di/service_locator.dart';

class AuthProvider extends ChangeNotifier {
  //final _api = getIt<ApiClient>();
  final _prefs = getIt<SharedPreferences>();

  bool _isLoading = false;
  String? _error;
  String? _token;
  bool _isInitialized = false;

  bool get isLoading => _isLoading;
  String? get error => _error;
  bool get isAuthenticated => _token != null;
  bool get isInitialized => _isInitialized;

  // 개발 중에는 임시로 항상 로그인되도록 설정
  Future<void> initialize() async {
    try {
      // 실제 구현 시에는 아래 주석을 해제
      // _token = _prefs.getString('auth_token');

      // 개발을 위한 임시 코드
      _token = 'temp_token';
      await _prefs.setString('auth_token', _token!);

      _isInitialized = true;
      notifyListeners();
    } catch (e) {
      _error = 'Failed to initialize auth: $e';
      _isInitialized = true;
      notifyListeners();
    }
  }

  Future<void> signInWithEmail(String email, String password) async {
    await _performAuthAction(() async {
      // TODO: 실제 API 연동 시 아래 주석 해제
      /*final response = await _api.post('/auth/login', {
        'email': email,
        'password': password,
      });
      await _handleAuthSuccess(response['token']);*/

      // 개발용 임시 코드
      await Future.delayed(const Duration(seconds: 1));
      await _handleAuthSuccess('temp_token');
    });
  }

  Future<void> signInWithGoogle() async {
    await _performAuthAction(() async {
      // 개발용 임시 코드
      await Future.delayed(const Duration(seconds: 1));
      await _handleAuthSuccess('google_token');
    });
  }

  Future<void> signInWithApple() async {
    await _performAuthAction(() async {
      await Future.delayed(const Duration(seconds: 1));
      await _handleAuthSuccess('apple_token');
    });
  }

  Future<void> signInWithKakao() async {
    await _performAuthAction(() async {
      await Future.delayed(const Duration(seconds: 1));
      await _handleAuthSuccess('kakao_token');
    });
  }

  Future<void> signOut() async {
    _token = null;
    await _prefs.remove('auth_token');
    notifyListeners();
  }

  Future<void> _performAuthAction(Future<void> Function() action) async {
    try {
      _isLoading = true;
      _error = null;
      notifyListeners();

      await action();
    } catch (e) {
      _error = 'Authentication failed: $e';
    } finally {
      _isLoading = false;
      notifyListeners();
    }
  }

  Future<void> _handleAuthSuccess(String token) async {
    _token = token;
    await _prefs.setString('auth_token', token);
  }
}
