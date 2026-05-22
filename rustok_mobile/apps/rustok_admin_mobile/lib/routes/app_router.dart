import 'package:app_module_contracts/app_module_contracts.dart';
import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';

import '../app_shell/app_shell_page.dart';
import '../registry/mobile_module_registry.dart';

GoRouter buildRouter(List<MobileModuleEntry> entries) {
  return GoRouter(
    initialLocation: '/modules',
    routes: [
      ShellRoute(
        builder: (context, state, child) => AppShellPage(child: child),
        routes: [
          GoRoute(
            path: '/modules',
            builder: (context, state) => const ModulesHomePage(),
            routes: [
              for (final entry in entries)
                GoRoute(
                  path: entry.routeSegment,
                  name: entry.moduleKey,
                  builder: (context, state) => ModulePlaceholderPage(entry: entry),
                ),
            ],
          ),
        ],
      ),
    ],
  );
}

class ModulesHomePage extends StatelessWidget {
  const ModulesHomePage({super.key});

  @override
  Widget build(BuildContext context) {
    final entries = mobileModuleRegistry;
    return ListView(
      children: [
        const ListTile(title: Text('RusTok Modules')),
        for (final entry in entries)
          ListTile(
            title: Text(entry.nav.title),
            subtitle: Text('/modules/${entry.routeSegment}'),
            onTap: () => context.go('/modules/${entry.routeSegment}'),
          ),
      ],
    );
  }
}

class ModulePlaceholderPage extends StatelessWidget {
  const ModulePlaceholderPage({super.key, required this.entry});

  final MobileModuleEntry entry;

  @override
  Widget build(BuildContext context) {
    return Center(
      child: Text('Module: ${entry.moduleKey}'),
    );
  }
}
