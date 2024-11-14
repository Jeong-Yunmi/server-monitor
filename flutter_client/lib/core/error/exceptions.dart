// flutter_client/lib/core/error/exceptions.dart
class ServerException implements Exception {
  final String message;
  ServerException([this.message = 'Server error occurred']);
}

class ServerTimeoutException implements Exception {
  final String message;
  ServerTimeoutException([this.message = 'Server connection timed out']);
}

class WebSocketException implements Exception {
  final String message;
  WebSocketException([this.message = 'WebSocket error occurred']);
}

class NetworkException implements Exception {
  final String message;
  NetworkException([this.message = 'Network error occurred']);
}

class CacheException implements Exception {
  final String message;
  CacheException([this.message = 'Cache error occurred']);
}

class AuthenticationException implements Exception {
  final String message;
  AuthenticationException([this.message = 'Authentication failed']);
}
