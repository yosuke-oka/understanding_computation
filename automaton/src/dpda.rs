use crate::pda_configuration::*;
use crate::state::State;

pub struct DPDARulebook {
    pub rules: Vec<PDARule>,
}

impl DPDARulebook {
    pub fn build(args: Vec<(State, char, State, char, Vec<char>)>) -> Self {
        DPDARulebook {
            rules: args
                .iter()
                .map(|t| PDARule::new(t.0, t.1, t.2, t.3, t.4.clone()))
                .collect(),
        }
    }
    pub fn next_configuration(
        &self,
        configuration: PDAConfiguration,
        character: char,
    ) -> PDAConfiguration {
        match self.rule_for(configuration.clone(), character) {
            Some(rule) => rule.follow(configuration),
            None => panic!("next_configuration is null"),
        }
    }
    fn rule_for(&self, configuration: PDAConfiguration, character: char) -> Option<&PDARule> {
        self.rules
            .iter()
            .find(|rule| rule.is_applied_to(configuration.clone(), character))
    }
}
