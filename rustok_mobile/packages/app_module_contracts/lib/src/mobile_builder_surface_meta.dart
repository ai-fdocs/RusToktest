class MobileBuilderSurfaceMeta {
  const MobileBuilderSurfaceMeta({
    required this.providerModule,
    required this.contract,
    required this.contractVersion,
    required this.builderContractVersion,
    this.capabilities = const <String>[],
    this.degradedModes = const <String, String>{},
    this.toggleProfiles = const <String, List<String>>{},
  });

  final String providerModule;
  final String contract;
  final String contractVersion;
  final String builderContractVersion;
  final List<String> capabilities;
  final Map<String, String> degradedModes;
  final Map<String, List<String>> toggleProfiles;
}
