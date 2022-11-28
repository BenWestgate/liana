use chrono::NaiveDateTime;

use iced::{
    alignment,
    widget::{Button, Column, Container, Row},
    Alignment, Element, Length,
};

use crate::ui::{
    component::{badge, button::Style, card, text::*},
    util::Collection,
};
use liana::miniscript::bitcoin;

use crate::{app::view::message::Message, daemon::model::HistoryTransaction};

pub const HISTORY_EVENT_PAGE_SIZE: u64 = 20;

pub fn home_view<'a>(
    balance: &'a bitcoin::Amount,
    pending_events: &[HistoryTransaction],
    events: &Vec<HistoryTransaction>,
) -> Element<'a, Message> {
    Column::new()
        .push(Column::new().padding(40))
        .push(text(format!("{} BTC", balance.to_btc())).bold().size(50))
        .push(
            Column::new()
                .spacing(10)
                .push(
                    pending_events
                        .iter()
                        .enumerate()
                        .fold(Column::new().spacing(10), |col, (i, event)| {
                            col.push(event_list_view(i, event))
                        }),
                )
                .push(
                    events
                        .iter()
                        .enumerate()
                        .fold(Column::new().spacing(10), |col, (i, event)| {
                            col.push(event_list_view(i, event))
                        }),
                )
                .push_maybe(
                    if events.len() % HISTORY_EVENT_PAGE_SIZE as usize == 0 && !events.is_empty() {
                        Some(
                            Container::new(
                                Button::new(
                                    text("See more")
                                        .width(Length::Fill)
                                        .horizontal_alignment(alignment::Horizontal::Center),
                                )
                                .width(Length::Fill)
                                .padding(15)
                                .style(Style::TransparentBorder.into())
                                .on_press(Message::Next),
                            )
                            .width(Length::Fill)
                            .style(card::SimpleCardStyle),
                        )
                    } else {
                        None
                    },
                ),
        )
        .align_items(Alignment::Center)
        .spacing(20)
        .into()
}

fn event_list_view<'a>(i: usize, event: &HistoryTransaction) -> Element<'a, Message> {
    Container::new(
        Button::new(
            Row::new()
                .push(
                    Row::new()
                        .push(if event.is_external() {
                            badge::receive()
                        } else {
                            badge::spend()
                        })
                        .push(if let Some(t) = event.time {
                            Container::new(
                                text(format!("{}", NaiveDateTime::from_timestamp(t as i64, 0)))
                                    .small(),
                            )
                        } else {
                            Container::new(text("  Pending  ").small())
                                .padding(3)
                                .style(badge::PillStyle::Success)
                        })
                        .spacing(10)
                        .align_items(Alignment::Center)
                        .width(Length::Fill),
                )
                .push(
                    Row::new()
                        .push(
                            text({
                                if event.is_external() {
                                    format!("+ {:.8}", event.incoming_amount.to_btc())
                                } else {
                                    format!("- {:.8}", event.outgoing_amount.to_btc())
                                }
                            })
                            .bold()
                            .width(Length::Shrink),
                        )
                        .push(text("BTC"))
                        .spacing(5)
                        .align_items(Alignment::Center),
                )
                .align_items(Alignment::Center)
                .spacing(20),
        )
        .padding(10)
        .on_press(Message::Select(i))
        .style(Style::TransparentBorder.into()),
    )
    .style(card::SimpleCardStyle)
    .into()
}

pub fn event_view<'a>(event: &HistoryTransaction) -> Element<'a, Message> {
    Column::new()
        .push(
            Row::new()
                .push(if event.is_external() {
                    badge::receive()
                } else {
                    badge::spend()
                })
                .spacing(10)
                .align_items(Alignment::Center),
        )
        .push(
            text({
                if event.is_external() {
                    format!("+ {} BTC", event.incoming_amount.to_btc())
                } else {
                    format!("- {} BTC", event.outgoing_amount.to_btc())
                }
            })
            .bold()
            .size(50)
            .width(Length::Shrink),
        )
        .push_maybe(
            event
                .fee_amount
                .map(|fee| Container::new(text(format!("Miner Fee: {} BTC", fee.to_btc())))),
        )
        .push(card::simple(
            Column::new()
                .push_maybe(event.time.map(|t| {
                    let date = NaiveDateTime::from_timestamp(t as i64, 0);
                    Row::new()
                        .width(Length::Fill)
                        .push(Container::new(text("Date:").bold()).width(Length::Fill))
                        .push(Container::new(text(format!("{}", date))).width(Length::Shrink))
                }))
                .push(
                    Row::new()
                        .width(Length::Fill)
                        .push(Container::new(text("Txid:").bold()).width(Length::Fill))
                        .push(
                            Container::new(text(format!("{}", event.tx.txid())))
                                .width(Length::Shrink),
                        ),
                )
                .spacing(5),
        ))
        .align_items(Alignment::Center)
        .spacing(20)
        .max_width(750)
        .into()
}
