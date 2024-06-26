use crate::ns::*;
use num_traits::ToPrimitive;

pub struct CssParser<'input> {
    tokenizer: CssTokenizer<'input>,
    previous_token: (Token, Location),
    token: (Token, Location),
    locations: Vec<Location>,
    expecting_token_error: bool,
}

impl<'input> CssParser<'input> {
    /// Constructs a tokenizer.
    pub fn new(compilation_unit: &'input Rc<CompilationUnit>, options: &ParserOptions) -> Self {
        Self {
            tokenizer: CssTokenizer::new(compilation_unit, options),
            previous_token: (Token::Eof, Location::with_offset(&compilation_unit, 0)),
            token: (Token::Eof, Location::with_offset(&compilation_unit, 0)),
            locations: vec![],
            expecting_token_error: false,
        }
    }

    fn options(&self) -> ParserOptions {
        ParserOptions {
            ..default()
        }
    }

    fn compilation_unit(&self) -> &Rc<CompilationUnit> {
        self.tokenizer.compilation_unit()
    }

    fn token_location(&self) -> Location {
        self.token.1.clone()
    }

    fn mark_location(&mut self) {
        self.locations.push(self.token.1.clone());
    }

    fn duplicate_location(&mut self) {
        self.locations.push(self.locations.last().unwrap().clone());
    }

    fn push_location(&mut self, location: &Location) {
        self.locations.push(location.clone());
    }

    fn pop_location(&mut self) -> Location {
        self.locations.pop().unwrap().combine_with(self.previous_token.1.clone())
    }

    fn add_syntax_error(&self, location: &Location, kind: DiagnosticKind, arguments: Vec<Rc<dyn DiagnosticArgument>>) {
        if self.compilation_unit().prevent_equal_offset_error(location) {
            return;
        }
        self.compilation_unit().add_diagnostic(Diagnostic::new_syntax_error(location, kind, arguments));
    }

    /*
    fn add_warning(&self, location: &Location, kind: DiagnosticKind, arguments: Vec<Rc<dyn DiagnosticArgument>>) {
        if self.compilation_unit().prevent_equal_offset_warning(location) {
            return;
        }
        self.compilation_unit().add_diagnostic(Diagnostic::new_warning(location, kind, arguments));
    }
    */

    fn next(&mut self) {
        self.previous_token = self.token.clone();
        self.token = self.tokenizer.scan();
    }

    fn peek(&self, token: Token) -> bool {
        self.token.0 == token
    }

    fn peek_identifier(&self) -> Option<(String, Location)> {
        if let Token::Identifier(id) = self.token.0.clone() {
            let location = self.token.1.clone();
            Some((id, location))
        } else {
            None
        }
    }

    fn peek_keyword(&self, name: &str) -> bool {
        if let Token::Identifier(id) = self.token.0.clone() { id == name && self.token.1.character_count() == name.len() } else { false }
    }

    fn consume(&mut self, token: Token) -> bool {
        if self.token.0 == token {
            self.next();
            true
        } else {
            false
        }
    }

    fn consume_identifier(&mut self) -> Option<(String, Location)> {
        if let Token::Identifier(id) = self.token.0.clone() {
            let location = self.token.1.clone();
            self.next();
            Some((id, location))
        } else {
            None
        }
    }

    fn consume_keyword(&mut self, name: &str) -> bool {
        if let Token::Identifier(name1) = self.token.0.clone() {
            if name1 == name {
                self.next();
                return true;
            }
        }
        false
    }

    /// Expects a token in non-greedy mode: if it fails, does not skip any token.
    fn expect(&mut self, token: Token) {
        if self.token.0 != token {
            self.expecting_token_error = true;
            self.add_syntax_error(&self.token_location(), DiagnosticKind::Expecting, diagarg![token.clone(), self.token.0.clone()]);
        } else {
            self.expecting_token_error = false;
            self.next();
        }
    }

    fn expect_identifier(&mut self) -> (String, Location) {
        if let Token::Identifier(id) = self.token.0.clone() {
            self.expecting_token_error = false;
            let location = self.token.1.clone();
            self.next();
            (id, location)
        } else {
            self.expecting_token_error = true;
            self.add_syntax_error(&self.token_location(), DiagnosticKind::ExpectingIdentifier, diagarg![self.token.0.clone()]);
            (INVALIDATED_IDENTIFIER.to_owned(), self.tokenizer.cursor_location())
        }
    }

    fn expect_unitless_number(&mut self) -> Option<f64> {
        if let Token::CssNumber { value, .. } = self.token.0.clone() {
            self.expecting_token_error = false;
            self.next();
            Some(value)
        } else {
            self.expecting_token_error = true;
            self.add_syntax_error(&self.token_location(), DiagnosticKind::Unexpected, diagarg![self.token.0.clone()]);
            None
        }
    }

    fn expect_string(&mut self) -> (String, Location) {
        if let Token::String(v) = self.token.0.clone() {
            self.expecting_token_error = false;
            let location = self.token.1.clone();
            self.next();
            (v, location)
        } else {
            self.expecting_token_error = true;
            self.add_syntax_error(&self.token_location(), DiagnosticKind::ExpectingStringLiteral, diagarg![self.token.0.clone()]);
            ("".into(), self.tokenizer.cursor_location())
        }
    }

    pub fn expect_eof(&mut self) {
        self.expect(Token::Eof);
    }

    fn create_invalidated_directive(&self, location: &Location) -> Rc<CssDirective> {
        Rc::new(CssDirective::Invalidated(InvalidatedNode {
            location: location.clone(),
        }))
    }

    fn create_invalidated_property_value(&self, location: &Location) -> Rc<CssPropertyValue> {
        Rc::new(CssPropertyValue::Invalidated(InvalidatedNode {
            location: location.clone(),
        }))
    }

    fn _create_invalidated_selector(&self, location: &Location) -> Rc<CssSelector> {
        Rc::new(CssSelector::Invalidated(InvalidatedNode {
            location: location.clone(),
        }))
    }

    fn create_invalidated_selector_condition(&self, location: &Location) -> Rc<CssSelectorCondition> {
        Rc::new(CssSelectorCondition::Invalidated(InvalidatedNode {
            location: location.clone(),
        }))
    }

    fn create_invalidated_media_query_condition(&self, location: &Location) -> Rc<CssMediaQueryCondition> {
        Rc::new(CssMediaQueryCondition::Invalidated(InvalidatedNode {
            location: location.clone(),
        }))
    }

    fn eof(&self) -> bool {
        matches!(self.token.0, Token::Eof)
    }

    pub fn parse_document(&mut self) -> Rc<CssDocument> {
        self.mark_location();
        let just_eof = self.peek(Token::Eof);
        let mut directives: Vec<Rc<CssDirective>> = vec![];
        while !self.eof() {
            directives.push(self.parse_directive());
        }
        let loc = self.pop_location();
        Rc::new(CssDocument {
            location: if just_eof {
                self.token.1.clone()
            } else {
                loc
            },
            directives,
        })
    }

    fn parse_directive(&mut self) -> Rc<CssDirective> {
        if let Some(rule) = self.parse_opt_rule() {
            Rc::new(CssDirective::Rule(rule))
        } else if self.peek(Token::CssAtNamespace) {
            self.mark_location();
            self.next();
            let prefix = self.expect_identifier();
            let uri = if self.expecting_token_error {
                (String::new(), self.tokenizer.cursor_location())
            } else {
                self.expect_string()
            };
            if !self.expecting_token_error {
                self.expect(Token::CssSemicolons);
            }
            let loc = self.pop_location();
            Rc::new(CssDirective::NamespaceDefinition(CssNamespaceDefinition {
                location: loc,
                prefix,
                uri,
            }))
        } else if self.peek(Token::CssAtMedia) {
            self.parse_media_query()
        } else if self.peek(Token::CssAtFontFace) {
            self.parse_font_face()
        } else {
            self.add_syntax_error(&self.token.1, DiagnosticKind::ExpectingDirective, diagarg![self.token.0.clone()]);
            let d = self.create_invalidated_directive(&self.tokenizer.cursor_location());
            self.next();
            d
        }
    }

    fn parse_media_query(&mut self) -> Rc<CssDirective> {
        self.mark_location();
        self.next();
        let mut conditions: Vec<Rc<CssMediaQueryCondition>> = vec![];
        let condition = self.parse_opt_media_query_condition();
        if let Some(condition) = condition {
            conditions.push(condition);
        } else {
            self.add_syntax_error(&self.token.1, DiagnosticKind::Unexpected, diagarg![self.token.0.clone()]);
        }
        loop {
            if let Some(condition) = self.parse_opt_media_query_condition() {
                conditions.push(condition);
            } else if self.eof() || self.peek(Token::BlockOpen) {
                break;
            } else if !self.consume(Token::Comma) {
                self.add_syntax_error(&self.token.1, DiagnosticKind::Unexpected, diagarg![self.token.0.clone()]);
                self.next();
            }
        }
        let mut rules: Vec<Rc<CssRule>> = vec![];
        self.expect(Token::BlockOpen);
        if !self.expecting_token_error {
            while !(self.eof() || self.peek(Token::BlockClose)) {
                if let Some(rule) = self.parse_opt_rule() {
                    rules.push(Rc::new(rule));
                } else {
                    self.add_syntax_error(&self.token.1, DiagnosticKind::Unexpected, diagarg![self.token.0.clone()]);
                    self.next();
                }
            }
            self.expect(Token::BlockClose);
        }
        Rc::new(CssDirective::MediaQuery(CssMediaQuery {
            location: self.pop_location(),
            conditions,
            rules,
        }))
    }

    fn parse_font_face(&mut self) -> Rc<CssDirective> {
        self.mark_location();
        self.next();
        let mut properties: Vec<Rc<CssProperty>> = vec![];
        self.expect(Token::BlockOpen);
        if !self.expecting_token_error {
            self.consume(Token::CssSemicolons);
            while !(self.eof() || self.peek(Token::BlockClose)) {
                properties.push(self.parse_property());
                if !self.consume(Token::CssSemicolons) {
                    break;
                }
            }
            self.expect(Token::BlockClose);
        }
        Rc::new(CssDirective::FontFace(CssFontFace {
            location: self.pop_location(),
            properties,
        }))
    }

    fn parse_opt_media_query_condition(&mut self) -> Option<Rc<CssMediaQueryCondition>> {
        let mut base: Option<Rc<CssMediaQueryCondition>> = None;
        if self.peek_keyword("only") {
            self.mark_location();
            self.next();
            let id = self.expect_identifier();
            base = Some(Rc::new(CssMediaQueryCondition::OnlyId {
                location: self.pop_location(),
                id,
            }));
        }
        if let Some(id) = self.consume_identifier() {
            base = Some(Rc::new(CssMediaQueryCondition::Id(id)));
        }
        if self.peek(Token::ParenOpen) {
            self.mark_location();
            let property = self.parse_arguments().unwrap().parse_property();
            let loc = self.pop_location();
            base = Some(Rc::new(CssMediaQueryCondition::ParenProperty((property, loc))));
        }
        if let Some(mut base) = base.clone() {
            while self.consume_keyword("and") {
                self.push_location(&base.location());
                if let Some(right) = self.parse_opt_media_query_condition() {
                    base = Rc::new(CssMediaQueryCondition::And {
                        location: self.pop_location(),
                        left: base,
                        right,
                    });
                } else {
                    self.add_syntax_error(&self.token.1, DiagnosticKind::Unexpected, diagarg![self.token.0.clone()]);
                    base = Rc::new(CssMediaQueryCondition::And {
                        location: self.pop_location(),
                        left: base,
                        right: self.create_invalidated_media_query_condition(&self.tokenizer.cursor_location()),
                    });
                }
            }
            return Some(base);
        }
        base
    }

    fn parse_arguments(&mut self) -> Result<CssParserFacade, ParserError> {
        if !self.peek(Token::ParenOpen) {
            self.add_syntax_error(&self.token.1, DiagnosticKind::Expecting, diagarg![Token::ParenOpen, self.token.0.clone()]);
            return Err(ParserError::Common);
        }
        let (byte_range, token) = self.tokenizer.scan_arguments();
        self.previous_token = self.token.clone();
        self.token = token;
        self.next();
        Ok(CssParserFacade(self.compilation_unit(), ParserOptions {
            byte_range: Some(byte_range),
            ..self.options()
        }))
    }

    fn parse_opt_rule(&mut self) -> Option<CssRule> {
        let mut selectors: Vec<Rc<CssSelector>> = vec![self.parse_opt_selector()?];
        while self.consume(Token::Comma) {
            if let Some(s) = self.parse_opt_selector() {
                selectors.push(s);
            } else {
                self.add_syntax_error(&self.token.1, DiagnosticKind::Unexpected, diagarg![self.token.0.clone()]);
            }
        }
        let mut properties: Vec<Rc<CssProperty>> = vec![];
        self.expect(Token::BlockOpen);
        if !self.expecting_token_error {
            self.consume(Token::CssSemicolons);
            while !(self.eof() || self.peek(Token::BlockClose)) {
                properties.push(self.parse_property());
                if !self.consume(Token::CssSemicolons) {
                    break;
                }
            }
            self.expect(Token::BlockClose);
        }
        self.push_location(&selectors[0].location());
        Some(CssRule {
            location: self.pop_location(),
            selectors,
            properties,
        })
    }

    fn parse_opt_selector(&mut self) -> Option<Rc<CssSelector>> {
        let mut base = self.parse_opt_base_selector()?;
        
        // Parse descendant combinators
        while let Some(right) = self.parse_opt_base_selector() {
            self.push_location(&base.location());
            base = Rc::new(CssSelector::Combinator(CssCombinatorSelector {
                location: self.pop_location(),
                left: base,
                right,
                combinator_type: CssCombinatorType::Descendant,
            }));
        }

        Some(base)
    }

    fn parse_opt_base_selector(&mut self) -> Option<Rc<CssSelector>> {
        self.mark_location();
        let mut namespace_prefix: Option<(String, Location)> = None;
        let mut element_name: Option<(String, Location)> = self.consume_identifier();
        let mut conditions: Vec<Rc<CssSelectorCondition>> = vec![];
        if self.consume(Token::Pipe) {
            namespace_prefix = element_name.clone();
            element_name = Some(self.expect_identifier());
        }
        // Parse conditions as long as they are not separated by whitespace
        while (element_name.is_none() && conditions.is_empty()) || (self.token.1.first_offset() - self.previous_token.1.last_offset() == 0) {
            if let Some(condition) = self.parse_opt_selector_condition() {
                conditions.push(condition);
            } else {
                break;
            }
        }
        if element_name.is_none() && conditions.is_empty() {
            self.pop_location();
            return None;
        }
        Some(Rc::new(CssSelector::Base(CssBaseSelector {
            location: self.pop_location(),
            namespace_prefix,
            element_name,
            conditions,
        })))
    }

    fn parse_selector_condition(&mut self) -> Rc<CssSelectorCondition> {
        let Some(c) = self.parse_opt_selector_condition() else {
            self.add_syntax_error(&self.token.1, DiagnosticKind::Unexpected, diagarg![self.token.0.clone()]);
            return self.create_invalidated_selector_condition(&self.tokenizer.cursor_location());
        };
        c
    }

    fn parse_opt_selector_condition(&mut self) -> Option<Rc<CssSelectorCondition>> {
        if self.peek(Token::Dot) {
            self.mark_location();
            self.next();
            let class_name = self.expect_identifier().0;
            return Some(Rc::new(CssSelectorCondition::Class((class_name, self.pop_location()))));
        }
        if let Token::CssHashWord(id_value) = self.token.0.clone() {
            let loc = self.token.1.clone();
            self.next();
            return Some(Rc::new(CssSelectorCondition::Id((id_value, loc))));
        }
        if self.peek(Token::Colon) {
            self.mark_location();
            self.next();
            if self.consume_keyword("not") {
                let condition = if let Ok(a) = self.parse_arguments() {
                    a.parse_selector_condition()
                } else {
                    self.duplicate_location();
                    let loc = self.pop_location();
                    self.create_invalidated_selector_condition(&loc)
                };
                return Some(Rc::new(CssSelectorCondition::Not {
                    location: self.pop_location(),
                    condition,
                }));
            } else {
                let name = self.expect_identifier().0;
                return Some(Rc::new(CssSelectorCondition::Pseudo((name, self.pop_location()))));
            }
        }
        if self.peek(Token::ColonColon) {
            self.mark_location();
            self.next();
            let name = self.expect_identifier().0;
            return Some(Rc::new(CssSelectorCondition::PseudoElement((name, self.pop_location()))));
        }
        if self.peek(Token::SquareOpen) {
            self.mark_location();
            self.next();
            let name = self.expect_identifier();
            let mut operator: Option<CssAttributeOperator> = None;
            let mut value: Option<(String, Location)> = None;
            while let Some(t1) = self.consume_attribute_operator() {
                operator = Some(t1);
            }
            while let Token::String(s1) = self.token.0.clone() {
                value = Some((s1, self.token.1.clone()));
                self.next();
            }
            self.expect(Token::SquareClose);
            return Some(Rc::new(CssSelectorCondition::Attribute {
                location: self.pop_location(),
                name,
                operator,
                value,
            }));
        }
        None
    }

    fn consume_attribute_operator(&mut self) -> Option<CssAttributeOperator> {
        if self.consume(Token::CssBeginsWith) {
            Some(CssAttributeOperator::BeginsWith)
        } else if self.consume(Token::CssEndsWith) {
            Some(CssAttributeOperator::EndsWith)
        } else if self.consume(Token::CssContains) {
            Some(CssAttributeOperator::Contains)
        } else if self.consume(Token::CssListMatch) {
            Some(CssAttributeOperator::ListMatch)
        } else if self.consume(Token::CssHreflangMatch) {
            Some(CssAttributeOperator::HreflangMatch)
        } else if self.consume(Token::Assign) {
            Some(CssAttributeOperator::Equals)
        } else {
            None
        }
    }

    fn parse_property(&mut self) -> Rc<CssProperty> {
        self.mark_location();
        let name = self.expect_identifier();
        let mut value = self.create_invalidated_property_value(&self.tokenizer.cursor_location());
        if !self.expecting_token_error {
            self.expect(Token::Colon);
            if !self.expecting_token_error {
                value = self.parse_property_value(CssOperatorPrecedence::Array);
            }
        }
        Rc::new(CssProperty::new(self.pop_location(), name, value))
    }

    fn parse_property_value(&mut self, min_precedence: CssOperatorPrecedence) -> Rc<CssPropertyValue> {
        let Some(v) = self.parse_opt_property_value(min_precedence) else {
            self.add_syntax_error(&self.token.1, DiagnosticKind::Unexpected, diagarg![self.token.0.clone()]);
            return self.create_invalidated_property_value(&self.tokenizer.cursor_location());
        };
        v
    }

    fn parse_opt_property_value(&mut self, min_precedence: CssOperatorPrecedence) -> Option<Rc<CssPropertyValue>> {
        let base: Option<Rc<CssPropertyValue>>;
        let t1 = self.token.0.clone();

        // #HHH
        // #HHHHHH
        if let Token::CssHashWord(word) = t1 {
            self.mark_location();
            self.next();
            let loc = self.pop_location();
            if let Ok(v) = CssColorPropertyValue::from_hex(loc.clone(), &word) {
                base = Some(Rc::new(CssPropertyValue::Color(v)));
            } else {
                base = Some(self.create_invalidated_property_value(&loc));
            }
        // "..."
        // '...'
        } else if let Token::String(value) = t1 {
            self.mark_location();
            self.next();
            base = Some(Rc::new(CssPropertyValue::String(CssStringPropertyValue {
                location: self.pop_location(),
                value
            })));
        // DECIMAL
        } else if let Token::CssNumber { value, unit } = t1 {
            self.mark_location();
            self.next();
            let loc = self.pop_location();
            base = Some(Rc::new(CssPropertyValue::Number(CssNumberPropertyValue {
                location: loc,
                value,
                unit,
            })));
        } else if let Some(id) = self.peek_identifier() {
            self.mark_location();
            self.next();
            let color_int = css_color_constant_to_int(&id.0);
            // COLOR_NAME such as "red"
            if let Some(color_int) = color_int {
                base = Some(Rc::new(CssPropertyValue::Color(CssColorPropertyValue {
                    location: self.pop_location(),
                    color_int,
                })));
            // rgb(...)
            } else if id.0 == "rgb" && self.peek(Token::ParenOpen) {
                if let Some(color_int) = self.parse_arguments().unwrap().parse_rgb() {
                    base = Some(Rc::new(CssPropertyValue::RgbColor(CssRgbColorPropertyValue {
                        location: self.pop_location(),
                        color_int,
                    })));
                } else {
                    let loc = self.pop_location();
                    base = Some(self.create_invalidated_property_value(&loc));
                }
            } else if id.0 == "ClassReference" && self.peek(Token::ParenOpen) {
                let name = self.parse_arguments().unwrap().parse_text();
                base = Some(Rc::new(CssPropertyValue::ClassReference(CssClassReferencePropertyValue {
                    location: self.pop_location(),
                    name,
                })));
            } else if id.0 == "PropertyReference" && self.peek(Token::ParenOpen) {
                let name = self.parse_arguments().unwrap().parse_text();
                base = Some(Rc::new(CssPropertyValue::PropertyReference(CssPropertyReferencePropertyValue {
                    location: self.pop_location(),
                    name,
                })));
            } else if id.0 == "url" && self.peek(Token::ParenOpen) {
                let url = self.parse_arguments().unwrap().parse_text();
                let mut format: Option<(String, Location)> = None;
                if self.consume_keyword("format") {
                    if let Ok(a) = self.parse_arguments() {
                        format = Some(a.parse_text());
                    }
                }
                base = Some(Rc::new(CssPropertyValue::Url(CssUrlPropertyValue {
                    location: self.pop_location(),
                    url,
                    format,
                })));
            } else if id.0 == "local" && self.peek(Token::ParenOpen) {
                let name = self.parse_arguments().unwrap().parse_text();
                base = Some(Rc::new(CssPropertyValue::Local(CssLocalPropertyValue {
                    location: self.pop_location(),
                    name,
                })));
            } else if id.0 == "Embed" && self.peek(Token::ParenOpen) {
                let entries = self.parse_arguments().unwrap().parse_embed_entries();
                base = Some(Rc::new(CssPropertyValue::Embed(CssEmbedPropertyValue {
                    location: self.pop_location(),
                    entries,
                })));
            } else {
                if self.peek(Token::ParenOpen) {
                    self.add_syntax_error(&self.token_location(), DiagnosticKind::Unexpected, diagarg![self.token.0.clone()]);
                    self.parse_arguments().unwrap();
                }
                base = Some(Rc::new(CssPropertyValue::Identifier(CssIdentifierPropertyValue {
                    location: self.pop_location(),
                    value: id.0,
                })));
            }
        } else if  self.peek(Token::Plus)  || self.peek(Token::Minus)
                || self.peek(Token::Times) || self.peek(Token::Div) {
            base = Some(self.create_invalidated_property_value(&self.token.1));
            self.next();
        } else {
            return None;
        }

        let mut base = base.unwrap();

        loop {
            if self.peek(Token::Comma) && min_precedence.includes(&CssOperatorPrecedence::Array) {
                self.push_location(&base.location());
                let mut elements: Vec<Rc<CssPropertyValue>> = vec![base];
                while self.consume(Token::Comma) {
                    elements.push(self.parse_property_value(CssOperatorPrecedence::MultiValue));
                }
                base = Rc::new(CssPropertyValue::Array(CssArrayPropertyValue {
                    location: self.pop_location(),
                    elements,
                }));
            } else if min_precedence.includes(&CssOperatorPrecedence::MultiValue) {
                if let Some(mv1) = self.parse_opt_property_value(CssOperatorPrecedence::MultiValue.add(1).unwrap()) {
                    self.push_location(&base.location());
                    let mut values: Vec<Rc<CssPropertyValue>> = vec![base, mv1];
                    while let Some(mv1) = self.parse_opt_property_value(CssOperatorPrecedence::MultiValue.add(1).unwrap()) {
                        values.push(mv1);
                    }
                    base = Rc::new(CssPropertyValue::MultiValue(CssMultiValuePropertyValue {
                        location: self.pop_location(),
                        values,
                    }));
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        Some(base)
    }

    fn parse_embed_entry(&mut self) -> Rc<CssEmbedEntry> {
        self.mark_location();
        if let Some(key) = self.consume_identifier() {
            if self.consume(Token::Assign) {
                let value = self.expect_string();
                Rc::new(CssEmbedEntry {
                    location: self.pop_location(),
                    key: Some(key),
                    value,
                })
            } else {
                Rc::new(CssEmbedEntry {
                    location: self.pop_location(),
                    key: None,
                    value: key,
                })
            }
        } else {
            let value = self.expect_string();
            Rc::new(CssEmbedEntry {
                location: self.pop_location(),
                key: None,
                value,
            })
        }
    }
}

fn rgb_bytes_to_integer(r: f64, g: f64, b: f64) -> u32 {
    (calc_rgb_byte(r) << 16) | (calc_rgb_byte(g) << 8) | calc_rgb_byte(b)
}

fn calc_rgb_byte(value: f64) -> u32 {
    // Integer
    if value.round() == value {
        value.round().to_u32().unwrap_or(0).clamp(0, 255)
    // Float
    } else {
        (value * 255.0).round().to_u32().unwrap_or(0).clamp(0, 255)
    }
}

/// A simplified interface for executing the CSS parser.
pub struct CssParserFacade<'input>(pub &'input Rc<CompilationUnit>, pub ParserOptions);

impl<'input> CssParserFacade<'input> {
    fn create_parser(&self) -> CssParser<'input> {
        CssParser::new(self.0, &self.1)
    }

    /// Parses `CssDocument` until end-of-file.
    pub fn parse_document(&self) -> Rc<CssDocument> {
        let mut parser = self.create_parser();
        parser.next();
        parser.parse_document()
    }

    /// Parses either a string or return source text as is.
    pub fn parse_text(&self) -> (String, Location) {
        let mut parser = self.create_parser();
        while parser.tokenizer.consume_whitespace() {
            // Consumed whitespace
        }
        let d = parser.tokenizer.characters().peek_or_zero();
        if ['"', '\''].contains(&d) {
            parser.next();
            let mut v: (String, Location) = ("".into(), parser.tokenizer.cursor_location());
            while let Token::String(v1) = parser.token.0.clone() {
                v = (v1, parser.token.1.clone());
                parser.next();
            }
            parser.expect_eof();
            v
        } else {
            let mut s = String::new();
            let i = parser.tokenizer.characters().index();
            while let Some(ch) = parser.tokenizer.characters_mut().next() {
                s.push(ch);
            }
            let j = parser.tokenizer.characters().index();
            (s, Location::with_offsets(parser.compilation_unit(), i, j))
        }
    }

    /// Parses `CssSelectorCondition` until end-of-file.
    pub fn parse_selector_condition(&self) -> Rc<CssSelectorCondition> {
        let mut parser = self.create_parser();
        parser.next();
        let r = parser.parse_selector_condition();
        parser.expect_eof();
        r
    }

    pub fn parse_property(&self) -> Rc<CssProperty> {
        let mut parser = self.create_parser();
        parser.next();
        let r = parser.parse_property();
        parser.expect_eof();
        r
    }

    pub fn parse_property_value(&self) -> Rc<CssPropertyValue> {
        let mut parser = self.create_parser();
        parser.next();
        let r = parser.parse_property_value(CssOperatorPrecedence::Array);
        parser.expect_eof();
        r
    }

    pub fn parse_rgb(&self) -> Option<u32> {
        let mut parser = self.create_parser();
        parser.next();
        let r = parser.expect_unitless_number()?;
        let g: f64;
        let b: f64;
        if parser.consume(Token::Comma) {
            g = parser.expect_unitless_number()?;
            parser.expect(Token::Comma);
            b = parser.expect_unitless_number()?;
        } else {
            g = parser.expect_unitless_number()?;
            b = parser.expect_unitless_number()?;
        }
        parser.expect_eof();
        Some(rgb_bytes_to_integer(r, g, b))
    }

    pub fn parse_embed_entries(&self) -> Vec<Rc<CssEmbedEntry>> {
        let mut parser = self.create_parser();
        let mut entries: Vec<Rc<CssEmbedEntry>> = vec![];
        parser.next();
        if !parser.eof() {
            entries.push(parser.parse_embed_entry());
        }
        while parser.consume(Token::Comma) {
            entries.push(parser.parse_embed_entry());
        }
        parser.expect_eof();
        entries
    }
}
