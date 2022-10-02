import 'package:flutter/widgets.dart';
import 'package:rusty_controller/bloc/specific_effect_bloc.dart';
import 'package:rusty_controller/model/led_effects.dart';

class StaticBloc
    extends SpecificEffectBloc<StaticEffectEvent, StaticLedEffect> {
  StaticBloc(StaticLedEffect effect) : super(effect) {
    on<StaticColorEvent>(
        (event, emit) => emit(state..color = event.currentColor));
  }
}

abstract class StaticEffectEvent {}

class StaticColorEvent extends StaticEffectEvent {
  HSVColor currentColor;

  double get initialValue => currentColor.value;

  StaticColorEvent(this.currentColor);
}
