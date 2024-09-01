
use liquid_core::Renderable;
use crate::error::Error;
use liquid_core::Runtime;

pub struct Template {
    pub compiled: liquid::Template,
}

impl Template {
    pub fn parse(template: String) ->  Result<Self, Error> {
        let maybe_template = liquid::ParserBuilder::with_stdlib()
            // filters
            .filter(crate::filters::aes::Aes256EncryptV2)
            .filter(crate::filters::aes::Aes256DecryptV2)
            .filter(crate::filters::aes::Aes256EncryptV1Deprecated)
            .filter(crate::filters::aes::Aes256DecryptV1Deprecated)
            .filter(crate::filters::hashing::Sha1)
            .filter(crate::filters::hashing::Sha256)
            .filter(crate::filters::hashing::Md5)
            .filter(crate::filters::array::Shuffle)
            .filter(crate::filters::string::Camelcase)
            .filter(crate::filters::string::AnyContains)
            .filter(crate::filters::string::EscapeNewline)
            .filter(crate::filters::url_encode::UrlEncode)
            .filter(crate::filters::url_encode::EscapeUrl)
            .filter(crate::filters::base64_filters::Base64Encode)
            .filter(crate::filters::base64_filters::Base64StrictEncode)
            .filter(crate::filters::base64_filters::B64Enc)
            .filter(crate::filters::base64_filters::Base64Decode)
            .filter(crate::filters::base64_filters::Base64StrictDecode)
            .filter(crate::filters::base64_filters::B64dec)
            .filter(crate::filters::money::Money)
            .filter(crate::filters::money::MoneyWithoutTrailingZeros)
            .filter(crate::filters::number::NumberWithDelimiter)
            .filter(crate::filters::number::NumberToPercentage)
            .filter(crate::filters::number::NumberWithPrecision)
            .filter(crate::filters::number::NumberToCurrency)
            .filter(crate::filters::number::NumberBetween)
            .filter(crate::filters::number::NumberMoreThan)
            .filter(crate::filters::number::NumberLessThan)
            .filter(crate::filters::timezone::TimeZone)
            .filter(crate::filters::to_json::ToJson)
            // tags
            .in_lax_mode()
            .build()
            .unwrap()
            .parse(&template);

        if let Err(err) = &maybe_template {
            return Err(Error::CompileError(
                err.to_string(),
            ));
        }

        Ok(Self {
            compiled: maybe_template.unwrap()
        })
    }

    pub fn render_with_context<T: Default + 'static>(
        &self,
        rc: T,
        globals: &liquid::model::Object,
    ) -> Result<String, Error>  {
        const BEST_GUESS: usize = 10_000;
        let mut buffer = Vec::with_capacity(BEST_GUESS);
        let runtime = liquid_core::runtime::RuntimeBuilder::new()
            .set_globals(globals)
            .set_render_mode(liquid_core::runtime::RenderingMode::Lax);
        let runtime = match self.compiled.partials {
            Some(ref partials) => runtime.set_partials(partials.as_ref()),
            None => runtime,
        };
        let runtime = runtime.build();

        {
        let mut cxt = runtime.registers().get_mut::<T>();
        *cxt = rc;
        }

        match self.compiled.template.render_to(&mut buffer, &runtime) {
            Ok(()) => Ok(convert_buffer(buffer)),
            Err(err) => {
                Err(Error::RenderingError(
                    err.to_string(),
                ))
            }
        }
    }
}

fn convert_buffer(buffer: Vec<u8>) -> String {
    unsafe { String::from_utf8_unchecked(buffer) }
}
