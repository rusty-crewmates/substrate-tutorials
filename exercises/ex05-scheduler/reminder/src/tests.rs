use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

use frame_support::traits::{OnFinalize, OnIdle, OnInitialize};

mod mint {
	use super::*;

	#[test]
	fn schedule() {
		new_test_ext().execute_with(|| {
			assert_ok!(Reminder::schedule_reminder(
				Origin::signed(ALICE),
				1,
				"test".as_bytes().to_vec(),
			));
			assert_eq!(
				Reminder::reminders(1),
				vec! {
					"test".as_bytes().to_vec()
				}
			)
		})
	}

	#[test]
	fn execution_and_cleanup() {
		new_test_ext().execute_with(|| {
			assert_ok!(Reminder::schedule_reminder(
				Origin::signed(ALICE),
				2,
				"test".as_bytes().to_vec(),
			));
			assert_ok!(Reminder::schedule_reminder(
				Origin::signed(ALICE),
				2,
				"test2".as_bytes().to_vec(),
			));
			<Reminder as OnInitialize<u64>>::on_initialize(2);
			System::assert_last_event(crate::Event::Reminder("test2".as_bytes().to_vec()).into());
			System::assert_has_event(crate::Event::Reminder("test".as_bytes().to_vec()).into());

			//check if events have been removed from the storage after being emitted
			assert_eq!(Reminder::reminders(2), <Vec<Vec<u8>>>::new())
		})
	}

	#[test]
	fn counting_events() {
		new_test_ext().execute_with(|| {
			assert_ok!(Reminder::schedule_reminder(
				Origin::signed(ALICE),
				2,
				"test".as_bytes().to_vec(),
			));
			assert_ok!(Reminder::schedule_reminder(
				Origin::signed(ALICE),
				2,
				"test2".as_bytes().to_vec(),
			));
			<Reminder as OnInitialize<u64>>::on_initialize(2);
			assert_eq!(Reminder::event_counter(), 2);
			<Reminder as OnIdle<u64>>::on_idle(2, 10000);
			System::assert_last_event(Event::Reminder(crate::Event::RemindersExecuteds(2)).into());
		})
	}

	#[test]
	fn reset_timer() {
		new_test_ext().execute_with(|| {
			assert_ok!(Reminder::schedule_reminder(
				Origin::signed(ALICE),
				2,
				"test".as_bytes().to_vec(),
			));
			assert_ok!(Reminder::schedule_reminder(
				Origin::signed(ALICE),
				2,
				"test2".as_bytes().to_vec(),
			));
			<Reminder as OnInitialize<u64>>::on_initialize(2);
			<Reminder as OnIdle<u64>>::on_idle(2, 10000);
			assert_eq!(Reminder::event_counter(), 2);
			<Reminder as OnFinalize<u64>>::on_finalize(2);
			assert_eq!(Reminder::event_counter(), 0);
		})
	}
}
