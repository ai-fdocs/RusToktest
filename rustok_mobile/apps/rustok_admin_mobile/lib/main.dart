import 'package:flutter/material.dart';

void main() {
  runApp(const RusTokAdminMobileApp());
}

class RusTokAdminMobileApp extends StatelessWidget {
  const RusTokAdminMobileApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'RusTok Admin Mobile',
      theme: ThemeData(useMaterial3: true, colorSchemeSeed: Colors.deepPurple),
      home: const Scaffold(
        body: Center(
          child: Text('RusTok mobile scaffold initialized'),
        ),
      ),
    );
  }
}
