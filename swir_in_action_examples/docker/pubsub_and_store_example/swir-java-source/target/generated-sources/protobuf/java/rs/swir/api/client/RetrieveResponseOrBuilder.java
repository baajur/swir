// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: client_api.proto

package rs.swir.api.client;

public interface RetrieveResponseOrBuilder extends
    // @@protoc_insertion_point(interface_extends:swir_public.RetrieveResponse)
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
   * <code>string database_name = 2;</code>
   * @return The databaseName.
   */
  java.lang.String getDatabaseName();
  /**
   * <code>string database_name = 2;</code>
   * @return The bytes for databaseName.
   */
  com.google.protobuf.ByteString
      getDatabaseNameBytes();

  /**
   * <code>string key = 3;</code>
   * @return The key.
   */
  java.lang.String getKey();
  /**
   * <code>string key = 3;</code>
   * @return The bytes for key.
   */
  com.google.protobuf.ByteString
      getKeyBytes();

  /**
   * <code>string status = 4;</code>
   * @return The status.
   */
  java.lang.String getStatus();
  /**
   * <code>string status = 4;</code>
   * @return The bytes for status.
   */
  com.google.protobuf.ByteString
      getStatusBytes();

  /**
   * <code>bytes payload = 5;</code>
   * @return The payload.
   */
  com.google.protobuf.ByteString getPayload();
}
