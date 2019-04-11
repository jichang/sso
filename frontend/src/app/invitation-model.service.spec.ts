import { TestBed } from "@angular/core/testing";

import { InvitationModelService } from "./invitation-model.service";

describe("InvitationsModelService", () => {
  beforeEach(() => TestBed.configureTestingModule({}));

  it("should be created", () => {
    const service: InvitationModelService = TestBed.get(InvitationModelService);
    expect(service).toBeTruthy();
  });
});
