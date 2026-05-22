class RouteSelection {
  const RouteSelection({required this.path, this.query = const {}});

  final String path;
  final Map<String, String> query;
}
