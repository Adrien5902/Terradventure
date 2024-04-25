use super::shop::{CurrentShop, Shop};
use crate::{gui::styles::text_style, state::AppState};
use bevy::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
/// La classe dialoge avec une liste de lignes de dialogues
pub struct Dialog {
    pub lines: Vec<DialogLine>,
}

impl Dialog {
    /// Vitesse d'apparition des caratères
    const CHARACTER_SPAWN_SPEED: f32 = 50.;
}

#[derive(Deserialize, Debug)]
/// Une ligne de dialoge
pub struct DialogLine {
    ///Avec une liste de choix/réponses possibles
    #[serde(default)]
    pub choices: Vec<DialogChoice>,
    ///Le message dit par le pnj
    pub message: String,
}

#[derive(Deserialize, Debug)]
/// Réponse à une ligne de dialogue
pub struct DialogChoice {
    /// Le texte de la réponse
    pub message: String,
    #[serde(default)]
    /// L'action déclenché quand on répond
    pub action: DialogChoiceAction,
}

#[derive(Clone, Deserialize, Default, Debug, Component)]
/// Action déclenché par la réponse
pub enum DialogChoiceAction {
    /// Aller à la prochaine ligne
    #[default]
    NextLine,
    /// Mettre fin au dialogue avec un message
    EndDialog(String),
    /// Aller à la ligne
    GotoLine(usize),
    /// Ouvrir un shop (magasin)
    OpenShop(String),
}

pub struct DialogPlugin;
impl Plugin for DialogPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, dialog_update.run_if(in_state(AppState::InGame)))
            .add_systems(OnExit(AppState::InGame), despawn_dialog_ui)
            .init_resource::<CurrentDialog>();
    }
}

#[derive(Resource, Default)]
pub struct CurrentDialog(pub Option<DialogResource>);

/// Données du dialogue en cours
pub struct DialogResource {
    /// Image du pnj
    pub orator_image: Handle<Image>,
    /// Nom du pnj
    pub orator_name: String,
    /// Objet dialogue avec les lignes
    pub dialog: Dialog,
    /// Numéro de la ligne actuelle
    pub line_index: isize,
}

#[derive(Component)]
pub struct DialogUi;

/// Etat du texte affiché sur l'ui
#[derive(Component)]
pub struct DialogUiText {
    /// Texte complet
    full_text: String,
    /// On garde le compte du nombre de caractères affichés pour afficher le texte petit à petit
    /// On utilise un flotant car on va multiplier le nombre par le delta time pour avoir la meme vitesse pour les utilisateur
    current_index: f32,
}

#[derive(Component)]
pub struct DialogUiTextContainer;

#[derive(Component)]
pub struct DialogUiChoicesContainer;

/// Fonction pour mettre à jour l'ui à chaque tick
fn dialog_update(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut current_dialog_res: ResMut<CurrentDialog>,
    dialog_ui_query: Query<Entity, With<DialogUi>>,
    mut text_query: Query<(&mut Text, &mut DialogUiText)>,
    text_container_query: Query<&Interaction, With<DialogUiTextContainer>>,
    choices_container_query: Query<Entity, With<DialogUiChoicesContainer>>,
    choices_query: Query<(&DialogChoiceAction, &Interaction)>,
    mut current_shop: ResMut<CurrentShop>,
    time: Res<Time>,
) {
    // On vérifie si on a un dialogue en cours
    let Some(current_dialog) = &mut current_dialog_res.0 else {
        if let Ok(dialog_ui_entity) = dialog_ui_query.get_single() {
            commands.entity(dialog_ui_entity).despawn_recursive();
        }

        // Si non on termine la fonction
        return;
    };

    // Si il y a un dialogue en cours et qu'il maque l'ui on la fait apparaitre
    if dialog_ui_query.get_single().is_err() {
        commands
            //Global
            .spawn(NodeBundle {
                background_color: Color::NONE.into(),
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::End,
                    width: Val::Vw(100.),
                    height: Val::Vh(100.),
                    ..Default::default()
                },
                z_index: ZIndex::Global(12),
                ..Default::default()
            })
            .insert(DialogUi)
            .with_children(|builder| {
                //Choices
                builder
                    .spawn(NodeBundle {
                        style: Style {
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::End,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(DialogUiChoicesContainer);

                //Text Container
                builder
                    .spawn(ButtonBundle {
                        background_color: Color::BLACK.with_a(0.7).into(),
                        style: Style {
                            width: Val::Percent(80.),
                            margin: UiRect::all(Val::Percent(2.)),
                            padding: UiRect::all(Val::Percent(2.)),
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .with_children(|builder| {
                        //NPC name
                        builder
                            .spawn(NodeBundle {
                                border_color: BorderColor(Color::WHITE),
                                style: Style {
                                    margin: UiRect::bottom(Val::Percent(2.)),
                                    border: UiRect::bottom(Val::Px(4.)),
                                    ..Default::default()
                                },
                                ..Default::default()
                            })
                            .with_children(|builder| {
                                builder.spawn(TextBundle::from_section(
                                    current_dialog.orator_name.clone(),
                                    TextStyle {
                                        font_size: 40.,
                                        ..text_style(&asset_server)
                                    },
                                ));
                            });

                        //Dialog text
                        builder
                            .spawn(TextBundle::from_section("", text_style(&asset_server)))
                            .insert(DialogUiText {
                                current_index: 0.,
                                full_text: String::default(),
                            });
                    })
                    .insert(DialogUiTextContainer);

                //NPC Image
                builder.spawn(ImageBundle {
                    image: UiImage::new(current_dialog.orator_image.clone()),
                    style: Style {
                        position_type: PositionType::Absolute,
                        right: Val::Percent(-18.),
                        bottom: Val::Percent(-35.),
                        width: Val::Vw(50.),
                        height: Val::Vw(50.),
                        ..Default::default()
                    },
                    ..Default::default()
                });
            });
    } else {
        //Sinon on mets a jour l'ui
        let (mut text, mut dialog_text) = text_query.single_mut();
        let choices_container = choices_container_query.single();

        let text_container_interaction = text_container_query.single();

        let line_index = current_dialog.line_index as usize;
        let current_line_opt = current_dialog.dialog.lines.get(line_index);

        if let Some(current_line) = current_line_opt {
            let dialog_needs_to_be_updated = dialog_text.full_text != current_line.message;

            if dialog_needs_to_be_updated {
                // On mets à jour l'état de l'ui voir [`DialogUiText`]
                dialog_text.full_text = current_line.message.clone();
                dialog_text.current_index = 0.;
                text.sections[0].value = String::default();

                //
                commands.entity(choices_container).despawn_descendants();
                for choice in &current_line.choices {
                    let choice_entity = commands
                        .spawn(ButtonBundle {
                            background_color: Color::BLACK.with_a(0.7).into(),
                            style: Style {
                                margin: UiRect::all(Val::Percent(1.)),
                                padding: UiRect::all(Val::Percent(1.)),
                                min_width: Val::Percent(30.),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|builder| {
                            builder.spawn(TextBundle::from_section(
                                &choice.message,
                                text_style(&asset_server),
                            ));
                        })
                        .insert(choice.action.clone())
                        .id();

                    commands.entity(choices_container).add_child(choice_entity);
                }
            }
        }

        // Si il n'y a pas de choix disponibles
        if !current_line_opt.is_some_and(|c| !c.choices.is_empty()) {

            // Et qu'on clique sur l'ui
            if *text_container_interaction == Interaction::Pressed {
                // Aller à la prochaine ligne
                next_line(&mut current_dialog_res);
            }
        } else {
            // On boucle sur les choix
            for (choice_action, interaction) in choices_query.iter() {
                // Si on clique sur ce choix
                if *interaction == Interaction::Pressed {
                    // On éxécute l'action du choix choissi
                    match choice_action {
                        DialogChoiceAction::EndDialog(message) => {
                            // On mets la ligne actuelle a -1 ça servira pour savoir que le dialogue sera fini a la prochaine ligne
                            current_dialog.line_index = -1;
                            // On mets le dernier message de fin sur l'ui
                            dialog_text.full_text = message.clone();
                            commands.entity(choices_container).despawn_descendants();
                        }
                        DialogChoiceAction::OpenShop(shop_name) => {
                            // On lit dans les assets le shop en question
                            let shop = Shop::read(shop_name);
                            // Et on change le shop ouvert actuel
                            *current_shop = CurrentShop { shop };

                            // On passe a la prochaine ligne 
                            // pour que l'utilisateur revienne sur le dialogue au bon moment un fois le shop fermé
                            next_line(&mut current_dialog_res);

                            return;
                        }
                        DialogChoiceAction::GotoLine(index) => {
                            current_dialog.line_index = *index as isize;
                        }
                        DialogChoiceAction::NextLine => {
                            next_line(&mut current_dialog_res);
                            return;
                        }
                    }
                }
            }
        }

        // Apparition petit a petit des caractères
        // Liste des caractères
        let chars: Vec<char> = dialog_text.full_text.chars().collect();
        // Numéro du caractère actuel arrondi
        let current_char_index = dialog_text.current_index as usize;
        
        // Tant qu'on a pas fini d'afficher tous les caractères
        // Pas besoin de `while` la fonction se répète déjà et on bloque le thread si on mettait un `while`
        if current_char_index <= chars.len() {
            // On augemente le nombre de caractères affiché
            // On multiplie par delta time pour avoir la meme vitesse pour tous les utilisateur
            dialog_text.current_index += Dialog::CHARACTER_SPAWN_SPEED * time.delta_seconds();
            // On récupère une coupe du texte qu'on mets mets dans le l'ui
            text.sections[0].value = chars[0..current_char_index].iter().collect();
        }
    }
}

fn despawn_dialog_ui(
    mut commands: Commands,
    dialog_ui_query: Query<Entity, With<DialogUi>>,
    mut current_dialog_res: ResMut<CurrentDialog>,
) {
    for entity in dialog_ui_query.iter() {
        commands.entity(entity).despawn_recursive();
        current_dialog_res.0 = None;
    }
}

// Aller à la prochaine ligne
fn next_line(current_dialog_res: &mut CurrentDialog) {
    let current_dialog = current_dialog_res.0.as_mut().unwrap();

    // Le numéro de dialogue et négatif le dialogue à été terminé par une action
    if current_dialog.line_index < 0 {
        // Terminer le dialogue
        current_dialog_res.0 = None;
        return;
    }

    // Si on a pas atteint la dernière ligne
    if (current_dialog.line_index as usize) < current_dialog.dialog.lines.len() {
        //Aller à la prochaine ligne
        current_dialog.line_index += 1;
    } else {
        // Terminer le dialogue
        current_dialog_res.0 = None;
    }
}

pub fn in_dialog(current_dialog: Res<CurrentDialog>) -> bool {
    current_dialog.0.is_some()
}
