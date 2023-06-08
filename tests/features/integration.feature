Feature: Integration

  Scenario: User Subscription
    When the user subscribes with username "<username>" and email "<email>"
    Then the database stored the username "<username>" and the email "<email>"

    Examples:
      | username         | email                     |
      | bob              | bob@acme.com              |
      | alice            | alice@acme.com            |
