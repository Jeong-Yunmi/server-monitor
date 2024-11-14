// flutter_client/lib/core/theme/app_theme.dart

import 'package:flutter/material.dart';

class AppTheme {
  static const double _borderRadius = 12.0;

  // Brand Colors
  static const Color _primaryLight = Color(0xFF536DFE);
  static const Color _primaryDark = Color(0xFF738AFF);
  static const Color _errorColor = Color(0xFFEF5350);
  static const Color _warningColor = Color(0xFFFFB74D);
  static const Color _successColor = Color(0xFF66BB6A);

  // Background Colors
  static const Color _backgroundDark = Color(0xFF121212);
  static const Color _backgroundLight = Color(0xFFFAFAFA);
  static const Color _surfaceDark = Color(0xFF1E1E1E);
  static const Color _surfaceLight = Color(0xFFFFFFFF);

  // Text Colors
  static const Color _textDark = Color(0xFFE0E0E0);
  static const Color _textLight = Color(0xFF212121);
  static const Color _textSecondaryDark = Color(0xFF9E9E9E);
  static const Color _textSecondaryLight = Color(0xFF757575);

  static ThemeData light() {
    return ThemeData(
      useMaterial3: true,
      brightness: Brightness.light,
      primaryColor: _primaryLight,
      scaffoldBackgroundColor: _backgroundLight,
      colorScheme: ColorScheme.light(
        primary: _primaryLight,
        error: _errorColor,
        surface: _surfaceLight,
        onSurface: _textLight,
      ),
      appBarTheme: AppBarTheme(
        backgroundColor: _surfaceLight,
        foregroundColor: _textLight,
        elevation: 0,
        centerTitle: true,
        titleTextStyle: TextStyle(
          color: _textLight,
          fontSize: 20,
          fontWeight: FontWeight.bold,
        ),
      ),
      cardTheme: CardTheme(
        color: _surfaceLight,
        elevation: 2,
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(_borderRadius),
        ),
      ),
      textTheme: TextTheme(
        titleLarge: TextStyle(
          color: _textLight,
          fontSize: 24,
          fontWeight: FontWeight.bold,
        ),
        titleMedium: TextStyle(
          color: _textLight,
          fontSize: 16,
          fontWeight: FontWeight.w600,
        ),
        bodyLarge: TextStyle(
          color: _textLight,
          fontSize: 16,
        ),
        bodyMedium: TextStyle(
          color: _textSecondaryLight,
          fontSize: 14,
        ),
      ),
      navigationBarTheme: NavigationBarThemeData(
        backgroundColor: _surfaceLight,
        indicatorColor: _primaryLight.withOpacity(0.1),
        labelTextStyle: MaterialStateProperty.resolveWith((states) {
          if (states.contains(MaterialState.selected)) {
            return TextStyle(
              color: _primaryLight,
              fontSize: 12,
              fontWeight: FontWeight.bold,
            );
          }
          return TextStyle(
            color: _textSecondaryLight,
            fontSize: 12,
          );
        }),
      ),
      inputDecorationTheme: InputDecorationTheme(
        filled: true,
        fillColor: _backgroundLight,
        border: OutlineInputBorder(
          borderRadius: BorderRadius.circular(_borderRadius),
          borderSide: BorderSide.none,
        ),
        contentPadding: const EdgeInsets.symmetric(
          horizontal: 16,
          vertical: 16,
        ),
      ),
      elevatedButtonTheme: ElevatedButtonThemeData(
        style: ElevatedButton.styleFrom(
          backgroundColor: _primaryLight,
          foregroundColor: Colors.white,
          padding: const EdgeInsets.symmetric(
            horizontal: 24,
            vertical: 12,
          ),
          shape: RoundedRectangleBorder(
            borderRadius: BorderRadius.circular(_borderRadius),
          ),
        ),
      ),
    );
  }

  static ThemeData dark() {
    return ThemeData(
      useMaterial3: true,
      brightness: Brightness.dark,
      primaryColor: _primaryDark,
      scaffoldBackgroundColor: _backgroundDark,
      colorScheme: ColorScheme.dark(
        primary: _primaryDark,
        error: _errorColor,
        surface: _surfaceDark,
        onSurface: _textDark,
      ),
      appBarTheme: AppBarTheme(
        backgroundColor: _surfaceDark,
        foregroundColor: _textDark,
        elevation: 0,
        centerTitle: true,
        titleTextStyle: TextStyle(
          color: _textDark,
          fontSize: 20,
          fontWeight: FontWeight.bold,
        ),
      ),
      cardTheme: CardTheme(
        color: _surfaceDark,
        elevation: 4,
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(_borderRadius),
        ),
      ),
      textTheme: TextTheme(
        titleLarge: TextStyle(
          color: _textDark,
          fontSize: 24,
          fontWeight: FontWeight.bold,
        ),
        titleMedium: TextStyle(
          color: _textDark,
          fontSize: 16,
          fontWeight: FontWeight.w600,
        ),
        bodyLarge: TextStyle(
          color: _textDark,
          fontSize: 16,
        ),
        bodyMedium: TextStyle(
          color: _textSecondaryDark,
          fontSize: 14,
        ),
      ),
      navigationBarTheme: NavigationBarThemeData(
        backgroundColor: _surfaceDark,
        indicatorColor: _primaryDark.withOpacity(0.1),
        labelTextStyle: MaterialStateProperty.resolveWith((states) {
          if (states.contains(MaterialState.selected)) {
            return TextStyle(
              color: _primaryDark,
              fontSize: 12,
              fontWeight: FontWeight.bold,
            );
          }
          return TextStyle(
            color: _textSecondaryDark,
            fontSize: 12,
          );
        }),
      ),
      inputDecorationTheme: InputDecorationTheme(
        filled: true,
        fillColor: _backgroundDark,
        border: OutlineInputBorder(
          borderRadius: BorderRadius.circular(_borderRadius),
          borderSide: BorderSide.none,
        ),
        contentPadding: const EdgeInsets.symmetric(
          horizontal: 16,
          vertical: 16,
        ),
      ),
      elevatedButtonTheme: ElevatedButtonThemeData(
        style: ElevatedButton.styleFrom(
          backgroundColor: _primaryDark,
          foregroundColor: Colors.white,
          padding: const EdgeInsets.symmetric(
            horizontal: 24,
            vertical: 12,
          ),
          shape: RoundedRectangleBorder(
            borderRadius: BorderRadius.circular(_borderRadius),
          ),
        ),
      ),
    );
  }

  // Utility Colors
  static Color getStatusColor(double value) {
    if (value >= 90) return _errorColor;
    if (value >= 80) return _warningColor;
    return _successColor;
  }

  static Color getServerStatusColor(bool isOnline) {
    return isOnline ? _successColor : _errorColor;
  }
}
