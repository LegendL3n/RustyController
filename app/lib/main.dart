import 'dart:async';

import 'package:flutter/material.dart';
import 'package:graphql_flutter/graphql_flutter.dart';
import 'package:logger/logger.dart';
import 'package:rusty_controller/extensions/color_extensions.dart';
import 'package:rusty_controller/model/led_effects.dart';
import 'package:rusty_controller/widgets/effect_chooser.dart';
import 'package:rusty_controller/widgets/effect_widget.dart';

var log = Logger(level: Level.debug, printer: PrettyPrinter());

// TODO: port to a service and DI
// TODO: use UDP discovery
var graphqlClient = GraphQLClient(
  link: HttpLink("http://127.0.0.1:8080/graphql"),
  cache: GraphQLCache(store: InMemoryStore()),
);

void main() => runApp(HomeScreen());

class HomeScreen extends StatelessWidget {
  final _effectChoiceController = StreamController<LedEffect>();

  final Map<EffectType, LedEffect> _effects = {
    EffectType.off: OffEffect(),
    EffectType.static: StaticEffect(color: Colors.black.toHSV()),
    EffectType.breathing:
        BreathingEffect(color: Colors.black.toHSV(), step: 0.01, peak: 1.0),
    EffectType.rainbow: RainbowEffect(saturation: 1.0, value: 1.0, step: 1),
  };

  HomeScreen({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        body: ScaffoldMessenger(
          child: StreamBuilder<LedEffect>(
            initialData: OffEffect(),
            stream: _effectChoiceController.stream,
            builder: (ctx, snapshot) {
              if (snapshot.hasError) {
                // TODO: banner here
              }

              if (!snapshot.hasData) {
                return const CircularProgressIndicator.adaptive();
              }

              final currentEffect = snapshot.data!;

              if (snapshot.connectionState == ConnectionState.active) {
                _effects[currentEffect.type] = currentEffect;
                graphqlClient
                    .mutate(
                      MutationOptions(
                          document: gql(currentEffect.graphqlMutation)),
                    )
                    .then(log.v, onError: (msg, _) => log.e(msg));
              }

              return SafeArea(
                child: Row(
                  children: [
                    Expanded(
                      child: EffectChooser(
                          effects: _effects,
                          choiceStream: _effectChoiceController.sink,
                          currentEffect: currentEffect),
                    ),
                    Expanded(
                      flex: 3,
                      child: EffectWidget(currentEffect,
                          colorStream: _effectChoiceController.sink),
                    ),
                  ],
                ),
              );
            },
          ),
        ),
      ),
    );
  }
}
