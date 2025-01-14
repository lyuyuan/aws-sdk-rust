// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[derive(std::fmt::Debug)]
pub(crate) struct Handle<C = aws_hyper::DynConnector> {
    client: aws_hyper::Client<C>,
    conf: crate::Config,
}

#[derive(Clone, std::fmt::Debug)]
pub struct Client<C = aws_hyper::DynConnector> {
    handle: std::sync::Arc<Handle<C>>,
}
impl<C> Client<C> {
    pub fn from_conf_conn(conf: crate::Config, conn: C) -> Self {
        let client = aws_hyper::Client::new(conn);
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }

    pub fn conf(&self) -> &crate::Config {
        &self.handle.conf
    }
}
impl Client {
    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn from_env() -> Self {
        Self::from_conf(crate::Config::builder().build())
    }

    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn from_conf(conf: crate::Config) -> Self {
        let client = aws_hyper::Client::https();
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }
}
impl<C> Client<C>
where
    C: aws_hyper::SmithyConnector,
{
    pub fn get_device_registration(&self) -> fluent_builders::GetDeviceRegistration<C> {
        fluent_builders::GetDeviceRegistration::new(self.handle.clone())
    }
    pub fn send_heartbeat(&self) -> fluent_builders::SendHeartbeat<C> {
        fluent_builders::SendHeartbeat::new(self.handle.clone())
    }
}
pub mod fluent_builders {
    #[derive(std::fmt::Debug)]
    pub struct GetDeviceRegistration<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::get_device_registration_input::Builder,
    }
    impl<C> GetDeviceRegistration<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::GetDeviceRegistrationOutput,
            smithy_http::result::SdkError<crate::error::GetDeviceRegistrationError>,
        >
        where
            C: aws_hyper::SmithyConnector,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>The unique name of the device you want to get the registration status from.</p>
        pub fn device_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.device_name(input);
            self
        }
        pub fn set_device_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_device_name(input);
            self
        }
        /// <p>The name of the fleet that the device belongs to.</p>
        pub fn device_fleet_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.device_fleet_name(input);
            self
        }
        pub fn set_device_fleet_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_device_fleet_name(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct SendHeartbeat<C = aws_hyper::DynConnector> {
        handle: std::sync::Arc<super::Handle<C>>,
        inner: crate::input::send_heartbeat_input::Builder,
    }
    impl<C> SendHeartbeat<C> {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::SendHeartbeatOutput,
            smithy_http::result::SdkError<crate::error::SendHeartbeatError>,
        >
        where
            C: aws_hyper::SmithyConnector,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>For internal use. Returns a list of SageMaker Edge Manager agent operating metrics.</p>
        pub fn agent_metrics(mut self, inp: impl Into<crate::model::EdgeMetric>) -> Self {
            self.inner = self.inner.agent_metrics(inp);
            self
        }
        pub fn set_agent_metrics(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::EdgeMetric>>,
        ) -> Self {
            self.inner = self.inner.set_agent_metrics(input);
            self
        }
        /// <p>Returns a list of models deployed on the the device.</p>
        pub fn models(mut self, inp: impl Into<crate::model::Model>) -> Self {
            self.inner = self.inner.models(inp);
            self
        }
        pub fn set_models(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::Model>>,
        ) -> Self {
            self.inner = self.inner.set_models(input);
            self
        }
        /// <p>Returns the version of the agent.</p>
        pub fn agent_version(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.agent_version(input);
            self
        }
        pub fn set_agent_version(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_agent_version(input);
            self
        }
        /// <p>The unique name of the device.</p>
        pub fn device_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.device_name(input);
            self
        }
        pub fn set_device_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_device_name(input);
            self
        }
        /// <p>The name of the fleet that the device belongs to.</p>
        pub fn device_fleet_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.device_fleet_name(input);
            self
        }
        pub fn set_device_fleet_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_device_fleet_name(input);
            self
        }
    }
}
