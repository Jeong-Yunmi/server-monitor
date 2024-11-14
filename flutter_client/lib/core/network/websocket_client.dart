// flutter_client/lib/core/network/websocket_client.dart

import 'dart:async';
import 'package:web_socket_channel/web_socket_channel.dart';
import '../error/exceptions.dart';

class WebSocketClient {
  WebSocketChannel? _channel;
  StreamController<Map<String, dynamic>>? _controller;
  Timer? _heartbeatTimer;
  bool _isConnected = false;

  Stream<Map<String, dynamic>>? get stream => _controller?.stream;
  bool get isConnected => _isConnected;

  Future<void> connect(String url) async {
    try {
      _channel = WebSocketChannel.connect(Uri.parse(url));
      _controller = StreamController<Map<String, dynamic>>.broadcast();

      _channel?.stream.listen(
        (data) {
          if (data != null) {
            _controller?.add(data as Map<String, dynamic>);
          }
        },
        onError: (error) {
          _controller?.addError(WebSocketException('Connection error: $error'));
          _handleDisconnect();
        },
        onDone: () {
          _handleDisconnect();
        },
      );

      _isConnected = true;
      _startHeartbeat();
    } catch (e) {
      throw WebSocketException('Failed to connect: $e');
    }
  }

  void _startHeartbeat() {
    _heartbeatTimer = Timer.periodic(
      const Duration(seconds: 30),
      (_) => sendMessage({'type': 'ping'}),
    );
  }

  void sendMessage(Map<String, dynamic> message) {
    if (!_isConnected) {
      throw WebSocketException('Not connected');
    }
    _channel?.sink.add(message);
  }

  void _handleDisconnect() {
    _isConnected = false;
    _heartbeatTimer?.cancel();
    _cleanup();
  }

  void _cleanup() {
    _channel?.sink.close();
    _controller?.close();
    _channel = null;
    _controller = null;
  }

  Future<void> disconnect() async {
    _handleDisconnect();
  }
}
