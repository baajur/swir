// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: client_api.proto

package rs.swir.api.client;

public interface SubscribeRequestOrBuilder extends
    // @@protoc_insertion_point(interface_extends:swir_public.SubscribeRequest)
    com.google.protobuf.MessageOrBuilder {

  /**
   * <pre>
   *correlation id for the whole stream
   * </pre>
   *
   * <code>string correlation_id = 1;</code>
   * @return The correlationId.
   */
  java.lang.String getCorrelationId();
  /**
   * <pre>
   *correlation id for the whole stream
   * </pre>
   *
   * <code>string correlation_id = 1;</code>
   * @return The bytes for correlationId.
   */
  com.google.protobuf.ByteString
      getCorrelationIdBytes();

  /**
   * <pre>
   *queue name, will be mapped to a physical queue name
   * </pre>
   *
   * <code>string topic = 2;</code>
   * @return The topic.
   */
  java.lang.String getTopic();
  /**
   * <pre>
   *queue name, will be mapped to a physical queue name
   * </pre>
   *
   * <code>string topic = 2;</code>
   * @return The bytes for topic.
   */
  com.google.protobuf.ByteString
      getTopicBytes();
}
