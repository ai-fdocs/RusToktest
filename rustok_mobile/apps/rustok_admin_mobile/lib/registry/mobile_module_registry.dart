import 'package:app_module_contracts/app_module_contracts.dart';

const mobileModuleRegistry = <MobileModuleEntry>[
  MobileModuleEntry(
    moduleKey: 'rustok_auth',
    routeSegment: 'auth',
    nav: MobileNavMeta(title: 'Auth', icon: 'shield'),
  ),
  MobileModuleEntry(
    moduleKey: 'rustok_blog',
    routeSegment: 'blog',
    nav: MobileNavMeta(title: 'Blog', icon: 'article'),
  ),
];
