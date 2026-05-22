import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import 'registry/mobile_module_registry.dart';
import 'routes/app_router.dart';

void main() {
  runApp(const ProviderScope(child: RusTokAdminMobileApp()));
}

class RusTokAdminMobileApp extends ConsumerWidget {
  const RusTokAdminMobileApp({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final router = buildRouter(mobileModuleRegistry);
    return MaterialApp.router(
      title: 'RusTok Admin Mobile',
      theme: ThemeData(useMaterial3: true, colorSchemeSeed: Colors.deepPurple),
      routerConfig: router,
    );
  }
}
