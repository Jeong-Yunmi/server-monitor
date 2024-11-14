// flutter_client/lib/core/api/api_client.dart

import 'package:dio/dio.dart';
import '../error/exceptions.dart';
import 'api_endpoints.dart';
import 'package:shared_preferences/shared_preferences.dart';

class ApiClient {
  final Dio _dio;
  final SharedPreferences _prefs;

  ApiClient(this._prefs) : _dio = Dio() {
    _dio.options.baseUrl = ApiEndpoints.baseUrl;
    _dio.options.connectTimeout = const Duration(seconds: 5);
    _dio.options.receiveTimeout = const Duration(seconds: 3);

    _dio.interceptors.add(InterceptorsWrapper(
      onRequest: (options, handler) async {
        // 저장된 인증 토큰이 있으면 헤더에 추가
        final token = _prefs.getString('auth_token');
        if (token != null) {
          options.headers['Authorization'] = 'Bearer $token';
        }
        return handler.next(options);
      },
      onError: (e, handler) {
        // DioException을 그대로 전달하고 나중에 처리
        return handler.next(e);
      },
    ));
  }

  Future<Map<String, dynamic>> get(String path) async {
    try {
      final response = await _dio.get(path);
      return response.data;
    } on DioException catch (e) {
      throw _handleError(e);
    }
  }

  Future<Map<String, dynamic>> post(
    String path,
    Map<String, dynamic> data,
  ) async {
    try {
      final response = await _dio.post(path, data: data);
      return response.data;
    } on DioException catch (e) {
      throw _handleError(e);
    }
  }

  Future<Map<String, dynamic>> put(
    String path,
    Map<String, dynamic> data,
  ) async {
    try {
      final response = await _dio.put(path, data: data);
      return response.data;
    } on DioException catch (e) {
      throw _handleError(e);
    }
  }

  Future<void> delete(String path) async {
    try {
      await _dio.delete(path);
    } on DioException catch (e) {
      throw _handleError(e);
    }
  }

  Exception _handleError(DioException e) {
    switch (e.type) {
      case DioExceptionType.connectionTimeout:
      case DioExceptionType.sendTimeout:
      case DioExceptionType.receiveTimeout:
        return ServerTimeoutException('Request timed out');

      case DioExceptionType.badResponse:
        final statusCode = e.response?.statusCode;
        final message =
            e.response?.data['message'] as String? ?? 'Unknown error';

        switch (statusCode) {
          case 400:
            return ServerException('Bad request: $message');
          case 401:
            _prefs.remove('auth_token'); // 인증 토큰 제거
            return AuthenticationException('Authentication failed');
          case 403:
            return ServerException('Access forbidden');
          case 404:
            return ServerException('Resource not found');
          case 500:
            return ServerException('Internal server error');
          default:
            return ServerException('Server error: $statusCode');
        }

      case DioExceptionType.connectionError:
        return NetworkException('Network connection error');

      default:
        return ServerException('An unexpected error occurred: ${e.message}');
    }
  }
}
