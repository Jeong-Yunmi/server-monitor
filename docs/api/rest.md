# REST API Documentation

## Overview
API endpoints for the server monitoring system.

## Authentication
All endpoints except `/health` and `/auth/login` require authentication via JWT token.

### Headers
```http
Authorization: Bearer <your_jwt_token>