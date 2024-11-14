// lib/features/server_list/widgets/add_server_dialog.dart

import 'package:flutter/material.dart';
import '../models/server_location.dart';

class AddServerDialog extends StatefulWidget {
  const AddServerDialog({Key? key}) : super(key: key);

  @override
  State<AddServerDialog> createState() => _AddServerDialogState();
}

class _AddServerDialogState extends State<AddServerDialog> {
  final _formKey = GlobalKey<FormState>();
  final _nameController = TextEditingController();
  String? _selectedProvider;
  CloudRegion? _selectedRegion;
  String? _selectedType;

  final _serverTypes = [
    'Production',
    'Development',
    'Staging',
  ];

  @override
  void dispose() {
    _nameController.dispose();
    super.dispose();
  }

  List<CloudRegion> _getRegionsForProvider(String? provider) {
    if (provider == null) return [];
    return cloudProviders.firstWhere((p) => p.code == provider).regions;
  }

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: const Text('Add New Server'),
      content: Form(
        key: _formKey,
        child: SingleChildScrollView(
          child: Column(
            mainAxisSize: MainAxisSize.min,
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              TextFormField(
                controller: _nameController,
                decoration: const InputDecoration(
                  labelText: 'Server Name',
                  hintText: 'Enter server name',
                ),
                validator: (value) {
                  if (value == null || value.isEmpty) {
                    return 'Please enter a server name';
                  }
                  return null;
                },
              ),
              const SizedBox(height: 16),
              const Text(
                'Cloud Provider',
                style: TextStyle(
                  fontSize: 12,
                  color: Colors.grey,
                ),
              ),
              DropdownButtonFormField<String>(
                value: _selectedProvider,
                hint: const Text('Select cloud provider'),
                items: [
                  const DropdownMenuItem(
                    value: 'custom',
                    child: Text('Custom Location'),
                  ),
                  ...cloudProviders.map((provider) => DropdownMenuItem(
                        value: provider.code,
                        child: Text(provider.name),
                      )),
                ],
                onChanged: (value) {
                  setState(() {
                    _selectedProvider = value;
                    _selectedRegion = null;
                  });
                },
                validator: (value) {
                  if (value == null) {
                    return 'Please select a provider';
                  }
                  return null;
                },
              ),
              const SizedBox(height: 16),
              if (_selectedProvider != null && _selectedProvider != 'custom')
                Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    const Text(
                      'Region',
                      style: TextStyle(
                        fontSize: 12,
                        color: Colors.grey,
                      ),
                    ),
                    DropdownButtonFormField<CloudRegion>(
                      value: _selectedRegion,
                      hint: const Text('Select region'),
                      items: _getRegionsForProvider(_selectedProvider)
                          .map((region) => DropdownMenuItem(
                                value: region,
                                child: Text(region.name),
                              ))
                          .toList(),
                      onChanged: (value) {
                        setState(() {
                          _selectedRegion = value;
                        });
                      },
                      validator: (value) {
                        if (_selectedProvider != 'custom' && value == null) {
                          return 'Please select a region';
                        }
                        return null;
                      },
                    ),
                  ],
                )
              else if (_selectedProvider == 'custom')
                TextFormField(
                  decoration: const InputDecoration(
                    labelText: 'Custom Location',
                    hintText: 'e.g., Seoul, KR',
                  ),
                  validator: (value) {
                    if (value == null || value.isEmpty) {
                      return 'Please enter a location';
                    }
                    return null;
                  },
                ),
              const SizedBox(height: 16),
              const Text(
                'Server Type',
                style: TextStyle(
                  fontSize: 12,
                  color: Colors.grey,
                ),
              ),
              DropdownButtonFormField<String>(
                value: _selectedType,
                hint: const Text('Select server type'),
                items: _serverTypes
                    .map((type) => DropdownMenuItem(
                          value: type,
                          child: Row(
                            children: [
                              Icon(
                                type == 'Production'
                                    ? Icons.public
                                    : type == 'Development'
                                        ? Icons.code
                                        : Icons
                                            .architecture, // staging 대신 architecture 아이콘 사용
                                size: 20,
                                color: type == 'Production'
                                    ? Colors.green
                                    : type == 'Development'
                                        ? Colors.blue
                                        : Colors.orange,
                              ),
                              const SizedBox(width: 8),
                              Text(type),
                            ],
                          ),
                        ))
                    .toList(),
                onChanged: (value) {
                  setState(() {
                    _selectedType = value;
                  });
                },
                validator: (value) {
                  if (value == null) {
                    return 'Please select a server type';
                  }
                  return null;
                },
              ),
            ],
          ),
        ),
      ),
      actions: [
        TextButton(
          onPressed: () => Navigator.pop(context),
          child: const Text('Cancel'),
        ),
        FilledButton(
          onPressed: () {
            if (_formKey.currentState!.validate()) {
              final location = _selectedProvider == 'custom'
                  ? 'Custom Location'
                  : // 실제로는 커스텀 입력값 사용
                  _selectedRegion?.name ?? 'Unknown';

              Navigator.pop(context, {
                'name': _nameController.text,
                'location': location,
                'type': _selectedType,
                'provider': _selectedProvider,
                'region': _selectedRegion?.id,
              });
            }
          },
          child: const Text('Add'),
        ),
      ],
    );
  }
}
