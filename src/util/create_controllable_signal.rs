use leptos::*;

pub(crate) struct CreateControllableSignalProps<T: Clone + PartialEq + 'static> {
  pub(crate) value: Signal<Option<T>>,
  pub(crate) default_value: Signal<Option<T>>,
  pub(crate) on_change: Callback<T>,
}

#[derive(Clone, Copy)]
pub(crate) struct WriteControllableSignal<T: Clone + 'static> {
  is_controlled: Signal<bool>,
  value: Signal<Option<T>>,
  pub(crate) set_uncontrolled_value: WriteSignal<Option<T>>,
  pub(crate) on_change: Callback<Option<T>>,
}

impl<T: Clone + 'static> WriteControllableSignal<T> {
  pub(crate) fn set(&self, value: T) {
    if self.is_controlled.get_untracked() {
      (self.on_change)(Some(value));
    } else {
      self.set_uncontrolled_value.set(Some(value.clone()));
      (self.on_change)(Some(value));
    }
  }

  pub(crate) fn update(&self, callback: impl FnOnce(&mut Option<T>)) {
    if self.is_controlled.get_untracked() {
      let mut value = self.value.get();

      callback(&mut value);
      (self.on_change)(value);
    } else {
      self.set_uncontrolled_value.update(|value| {
        callback(value);
        (self.on_change)(value.clone());
      });
    }
  }
}

pub(crate) fn create_controllable_signal<T: Clone + PartialEq + 'static>(
  CreateControllableSignalProps {
    value,
    default_value,
    on_change,
  }: CreateControllableSignalProps<T>,
) -> (Signal<Option<T>>, WriteControllableSignal<T>) {
  let (uncontrolled_value, set_uncontrolled_value) =
    create_uncontrolled_signal(CreateUncontrolledSignalProps {
      default_value,
      on_change,
    });

  let is_controlled = Signal::derive(move || value.get().is_some());
  let value = Signal::derive(move || {
    if is_controlled() {
      value.get()
    } else {
      uncontrolled_value.get()
    }
  });

  (
    value,
    WriteControllableSignal {
      is_controlled,
      value,
      set_uncontrolled_value,
      on_change: Callback::new(move |value| {
        if let Some(value) = value {
          on_change(value);
        }
      }),
    },
  )
}

pub(crate) struct CreateUncontrolledSignalProps<T: Clone + 'static> {
  default_value: Signal<Option<T>>,
  on_change: Callback<T>,
}

fn create_uncontrolled_signal<T: Clone + PartialEq + 'static>(
  CreateUncontrolledSignalProps {
    default_value,
    on_change,
  }: CreateUncontrolledSignalProps<T>,
) -> (ReadSignal<Option<T>>, WriteSignal<Option<T>>) {
  let (uncontrolled_value, set_uncontrolled_value) = create_signal(default_value.get());

  let prev_value = store_value(uncontrolled_value.get());

  create_effect(move |_| {
    if prev_value.get_value() != uncontrolled_value.get() {
      if let Some(value) = uncontrolled_value.get() {
        on_change(value);
      }

      prev_value.set_value(uncontrolled_value.get());
    }
  });

  (uncontrolled_value, set_uncontrolled_value)
}
