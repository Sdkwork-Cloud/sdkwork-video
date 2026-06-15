import 'package:flutter/material.dart';

class SdkWorkVideoApp extends StatelessWidget {
  const SdkWorkVideoApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'SDKWork Video',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.deepPurple),
        useMaterial3: true,
      ),
      home: const Scaffold(
        body: Center(
          child: Text('SDKWork Video Flutter Mobile'),
        ),
      ),
    );
  }
}