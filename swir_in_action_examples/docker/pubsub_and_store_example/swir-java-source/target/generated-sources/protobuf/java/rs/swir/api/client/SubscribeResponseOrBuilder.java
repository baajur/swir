// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: client_api.proto

package rs.swir.api.client;

public interface SubscribeResponseOrBuilder extends
    // @@protoc_insertion_point(interface_extends:swir_public.SubscribeResponse)
    com.google.protobuf.MessageOrBuilder {

  /**
   * <code>string correlation_id = 1;</code>
   * @return The correlationId.
   */
  java.lang.String getCorrelationId();
  /**
   * <code>string correlation_id = 1;</code>
   * @return The bytes for correlationId.
   */
  com.google.protobuf.ByteString
      getCorrelationIdBytes();

  /**
   * <pre>
   *Payload is copied directly into appropriate format
   * </pre>
   *
   * <code>bytes payload = 2;</code>
   * @return The payload.
   */
  com.google.protobuf.ByteString getPayload();
}
