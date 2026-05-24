import 'package:app_module_contracts/app_module_contracts.dart';

const modulesRootPath = '/modules';

class ModuleRouteEntry {
  const ModuleRouteEntry({
    required this.moduleKey,
    required this.routeSegment,
    required this.path,
    required this.navTitle,
    required this.childRoutes,
  });

  final String moduleKey;
  final String routeSegment;
  final String path;
  final String navTitle;
  final List<ModuleChildRouteEntry> childRoutes;
}

class ModuleChildRouteEntry {
  const ModuleChildRouteEntry({
    required this.subpath,
    required this.path,
    required this.title,
    required this.navLabel,
  });

  final String subpath;
  final String path;
  final String title;
  final String navLabel;
}

List<ModuleRouteEntry> adaptModuleEntries(List<MobileModuleEntry> entries) {
  final adapted = <ModuleRouteEntry>[];
  final usedModuleKeys = <String>{};
  final usedRouteSegments = <String>{};

  for (final entry in entries) {
    final moduleKey = entry.moduleKey.trim();
    final routeSegment = _sanitizeSegment(entry.routeSegment);
    if (moduleKey.isEmpty || routeSegment.isEmpty) {
      continue;
    }
    if (!usedModuleKeys.add(moduleKey) || !usedRouteSegments.add(routeSegment)) {
      continue;
    }

    final basePath = '$modulesRootPath/$routeSegment';
    final childRoutes = <ModuleChildRouteEntry>[];
    final usedChildSubpaths = <String>{};

    for (final child in entry.childPages) {
      final subpath = _sanitizeSegment(child.subpath);
      if (subpath.isEmpty || !usedChildSubpaths.add(subpath)) {
        continue;
      }

      childRoutes.add(
        ModuleChildRouteEntry(
          subpath: subpath,
          path: '$basePath/$subpath',
          title: child.title,
          navLabel: child.navLabel ?? child.title,
        ),
      );
    }

    adapted.add(
      ModuleRouteEntry(
        moduleKey: moduleKey,
        routeSegment: routeSegment,
        path: basePath,
        navTitle: entry.nav.title,
        childRoutes: List.unmodifiable(childRoutes),
      ),
    );
  }

  return List.unmodifiable(adapted);
}

String _sanitizeSegment(String value) {
  return value.trim().replaceAll(RegExp(r'^/+|/+$'), '');
}
