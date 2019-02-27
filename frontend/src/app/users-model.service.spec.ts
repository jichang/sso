import { TestBed } from '@angular/core/testing';

import { UsersModelService } from './users-model.service';

describe('UsersModelService', () => {
  beforeEach(() => TestBed.configureTestingModule({}));

  it('should be created', () => {
    const service: UsersModelService = TestBed.get(UsersModelService);
    expect(service).toBeTruthy();
  });
});
