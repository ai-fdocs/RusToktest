//! Property-based tests for core validation invariants.

#[cfg(test)]
mod tenant_validation_tests {
    use crate::tenant_validation::{TenantIdentifierValidator, TenantValidationError};
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn valid_slug_pattern_is_accepted(s in "[a-z0-9][a-z0-9-]{0,62}") {
            let result = TenantIdentifierValidator::validate_slug(&s);
            if matches!(result, Err(TenantValidationError::Reserved(_))) {
                prop_assert!(true);
            } else {
                prop_assert!(result.is_ok());
            }
        }

        #[test]
        fn empty_slug_is_rejected(_ in Just(())) {
            let result = TenantIdentifierValidator::validate_slug("");
            prop_assert!(matches!(result, Err(TenantValidationError::Empty)));
        }

        #[test]
        fn uppercase_slug_is_normalized(s in "[A-Z][A-Z0-9-]{0,20}") {
            let result = TenantIdentifierValidator::validate_slug(&s);
            if let Ok(normalized) = result {
                prop_assert_eq!(normalized, s.to_lowercase());
            }
        }

        #[test]
        fn uuid_validation_accepts_non_nil(uuid in any::<[u8; 16]>()) {
            prop_assume!(uuid != [0u8; 16]);
            let uuid_str = uuid::Uuid::from_bytes(uuid).to_string();
            prop_assert!(TenantIdentifierValidator::validate_uuid(&uuid_str).is_ok());
        }
    }
}

#[cfg(test)]
mod event_validation_tests {
    use crate::events::validation::{validators, EventValidationError, ValidateEvent};
    use crate::DomainEvent;
    use proptest::prelude::*;
    use uuid::Uuid;

    proptest! {
        #[test]
        fn validate_not_empty_accepts_non_empty(s in "[a-zA-Z0-9]{1,64}") {
            prop_assert!(validators::validate_not_empty("field", &s).is_ok());
        }

        #[test]
        fn validate_not_empty_rejects_whitespace(s in "[ \t\n\r]{1,10}") {
            prop_assert!(matches!(
                validators::validate_not_empty("field", &s),
                Err(EventValidationError::EmptyField(_))
            ));
        }

        #[test]
        fn node_created_kind_len_respected(kind in "[a-z]{1,100}") {
            let event = DomainEvent::NodeCreated {
                node_id: Uuid::new_v4(),
                kind: kind.clone(),
                author_id: None,
            };

            if kind.len() <= 64 {
                prop_assert!(event.validate().is_ok());
            } else {
                prop_assert!(event.validate().is_err());
            }
        }

        #[test]
        fn nil_node_id_rejected(_ in Just(())) {
            let event = DomainEvent::NodeCreated {
                node_id: Uuid::nil(),
                kind: "article".to_string(),
                author_id: None,
            };
            prop_assert!(matches!(event.validate(), Err(EventValidationError::NilUuid(_))));
        }
    }
}

#[cfg(test)]
mod event_serialization_tests {
    use crate::{DomainEvent, EventEnvelope};
    use proptest::prelude::*;
    use uuid::Uuid;

    proptest! {
        #[test]
        fn event_roundtrip(kind in "[a-z]{1,40}") {
            let original = DomainEvent::NodeCreated {
                node_id: Uuid::new_v4(),
                kind,
                author_id: Some(Uuid::new_v4()),
            };

            let json = serde_json::to_string(&original).unwrap();
            let decoded: DomainEvent = serde_json::from_str(&json).unwrap();
            prop_assert_eq!(decoded, original);
        }

        #[test]
        fn envelope_has_required_fields(_ in Just(())) {
            let envelope = EventEnvelope::new(
                Uuid::new_v4(),
                Some(Uuid::new_v4()),
                DomainEvent::NodeCreated {
                    node_id: Uuid::new_v4(),
                    kind: "article".to_string(),
                    author_id: None,
                },
            );

            let json = serde_json::to_string(&envelope).unwrap();
            let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
            prop_assert!(parsed.get("id").is_some());
            prop_assert!(parsed.get("event_type").is_some());
            prop_assert!(parsed.get("tenant_id").is_some());
            prop_assert!(parsed.get("event").is_some());
        }
    }
}
