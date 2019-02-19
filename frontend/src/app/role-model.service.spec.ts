import { TestBed } from '@angular/core/testing';

import { RoleModelService } from './role-model.service';

describe('RoleModelService', () => {
  beforeEach(() => TestBed.configureTestingModule({}));

  it('should be created', () => {
    const service: RoleModelService = TestBed.get(RoleModelService);
    expect(service).toBeTruthy();
  });
});
